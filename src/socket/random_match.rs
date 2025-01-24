
use socketioxide::{extract::{ Extension, SocketRef, State}, layer::SocketIoLayer, SocketIo};
use crate::{models::auth_user::AuthUser, services::auth_service::verify_token, AppState};
pub fn random_match_socket(app_state: AppState) -> SocketIoLayer {
    let (layer, io) = SocketIo::builder().with_state(app_state.clone()).build_layer();
    let mut receiver= app_state.broadcaster.subscribe();
   
    io.ns("/random-match", move |socket: SocketRef, State(app_state): State<AppState>| {
        let token = socket.req_parts().headers.get("authorization");
        if token.is_none() {
            socket.disconnect().unwrap_or_default();
            return;
        }
        let token = token.unwrap().to_str().unwrap().to_string(); // Clone the token for async use
        let app_state_clone = app_state.clone(); // Clone state for async closure

        // Use async task
        tokio::spawn(async move {
            let user = verify_token(&token, app_state_clone).await;

            if user.is_err() {
                socket.disconnect().unwrap_or_default();
                return;
            }

            let user = user.unwrap();
            socket.extensions.insert(user);
            socket.on("join_random_match", join_random_match);
        });
    });
    tokio::spawn(async move {
        loop {
            let message = receiver.recv().await.unwrap();
            let u_id=message.user_id.to_string();
            let user_room=format!("random-match-{u_id}");
            println!("{:?}",message);
            io.of("/random-match").unwrap().within(user_room).emit("on_matched", &message).unwrap();
        }
    });

    layer
}

pub async fn join_random_match(socket:SocketRef,
 State(_):State<AppState>,
 Extension(user):Extension<AuthUser>
){
    let u_id=user.id.to_string();
    let user_room=format!("random-match-{u_id}");
    socket.join(user_room.clone()).unwrap();
    socket.within(user_room.clone()).emit::<AuthUser>("join_random_match",&user).unwrap();
}