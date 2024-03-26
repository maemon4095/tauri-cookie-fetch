use bytes::Bytes;
use futures::{
    channel::mpsc::{channel, Receiver, Sender},
    SinkExt,
};
use pigeonhole::VecPigeonhole;
use reqwest::header;
use std::sync::Mutex;
use tauri::{http::status::StatusCode, Manager, State};

pub struct IpcSession {
    pub response_sender: futures::channel::mpsc::Sender<Bytes>,
    pub request_receiver: futures::channel::mpsc::Receiver<Vec<u8>>,
}

pub struct IpcSessionInternal {
    response_receiver: Receiver<Bytes>,
    request_sender: Sender<Vec<u8>>,
    open_session: Option<IpcSession>,
}

pub struct IpcState {
    sessions: Mutex<VecPigeonhole<IpcSessionInternal>>,
}

impl IpcState {
    pub fn session(&self, id: usize) -> Result<IpcSession, SessionError> {
        let mut sessions = self.sessions.lock().unwrap();
        let Some(session) = sessions.get_mut(id) else {
            return Err(SessionError::Unestablished);
        };

        let open_session = session.open_session.take();

        match open_session {
            Some(s) => Ok(s),
            None => Err(SessionError::Consumed),
        }
    }
}

#[tauri::command]
async fn connect(state: State<'_, IpcState>) -> Result<usize, String> {
    let (response_sender, response_receiver) = channel(32);
    let (request_sender, request_receiver) = channel(32);

    let mut sessions = state.sessions.lock().unwrap();
    let reservation = sessions.reserve();
    let id = reservation.id();

    let session = IpcSession {
        response_sender,
        request_receiver,
    };

    reservation.set(IpcSessionInternal {
        response_receiver,
        request_sender,
        open_session: Some(session),
    });

    Ok(id)
}

pub fn init<R: tauri::Runtime>(builder: tauri::plugin::Builder<R>) -> tauri::plugin::Builder<R> {
    builder
        .setup(|app| {
            app.manage(IpcState {
                sessions: Mutex::new(VecPigeonhole::new()),
            });
            Ok(())
        })
        .register_uri_scheme_protocol("cookie-fetch-ipc", |app, req| {
            if req.method() != tauri::http::method::Method::POST {
                return Err(MethodMustBePostError.into());
            }

            let uri: tauri::http::Uri = req.uri().parse()?;
            let path: RequestPath = uri.path().parse()?;

            let state = app.state::<IpcState>();
            let mut sessions = state.sessions.lock().unwrap();

            match path {
                RequestPath::Push(id) => {
                    let Some(session) = sessions.get_mut(id) else {
                        return Err(InvalidSessionId(id).into());
                    };

                    tauri::async_runtime::block_on(async {
                        session.request_sender.send(req.body().clone()).await
                    })?;

                    tauri::http::ResponseBuilder::new()
                        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                        .status(StatusCode::ACCEPTED)
                        .body(Vec::new())
                }
                RequestPath::Pop(id) => {
                    let Some(session) = sessions.get_mut(id) else {
                        return Err(InvalidSessionId(id).into());
                    };

                    let result = session.response_receiver.try_next();

                    let res = tauri::http::ResponseBuilder::new()
                        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*");

                    match result {
                        Ok(Some(chunk)) => res.status(StatusCode::OK).body(chunk.to_vec()),
                        Ok(None) => res.status(StatusCode::NO_CONTENT).body(Vec::new()),
                        Err(_) => res.status(StatusCode::CONTINUE).body(Vec::new()),
                    }
                }
                RequestPath::CloseDownstream(id) => {
                    let Some(session) = sessions.get_mut(id) else {
                        return Err(InvalidSessionId(id).into());
                    };

                    session.response_receiver.close();

                    tauri::http::ResponseBuilder::new()
                        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                        .status(StatusCode::OK)
                        .body(Vec::new())
                }
                RequestPath::CloseUpstream(id) => {
                    let Some(session) = sessions.get_mut(id) else {
                        return Err(InvalidSessionId(id).into());
                    };

                    session.request_sender.close_channel();

                    tauri::http::ResponseBuilder::new()
                        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                        .status(StatusCode::OK)
                        .body(Vec::new())
                }
            }
        })
        .invoke_handler(tauri::generate_handler![connect])
}

enum RequestPath {
    CloseDownstream(usize),
    CloseUpstream(usize),
    Push(usize),
    Pop(usize),
}

impl std::str::FromStr for RequestPath {
    type Err = RequestPathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix('/') else {
            return Err(RequestPathError);
        };

        let Some((id, method)) = s.split_once('/') else {
            return Err(RequestPathError);
        };

        let id = id.parse().map_err(|_| RequestPathError)?;
        match method {
            "push" => Ok(RequestPath::Push(id)),
            "pop" => Ok(RequestPath::Pop(id)),
            "close/downstream" => Ok(RequestPath::CloseDownstream(id)),
            "close/upstream" => Ok(RequestPath::CloseUpstream(id)),
            _ => Err(RequestPathError),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("cookie-fetch-ipc session id is invalid. id = {}.", self.0)]
pub struct InvalidSessionId(pub usize);

#[derive(Debug, thiserror::Error)]
#[error("the cookie-fetch-ipc request URI path is invalid.")]
struct RequestPathError;

#[derive(Debug, thiserror::Error)]
#[error("cookie-fetch-ipc method must be POST.")]
struct MethodMustBePostError;

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("cookie-fetch-ipc try to access unestablished session.")]
    Unestablished,
    #[error("cookie-fetch-ipc try to use already consumed session.")]
    Consumed,
}
