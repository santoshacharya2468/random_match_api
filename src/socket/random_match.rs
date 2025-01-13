
use socketioxide::{extract::{ Extension, SocketRef, State}, layer::SocketIoLayer, SocketIo};
use crate::{models::auth_user::AuthUser, services::auth_service::verify_token, AppState};
pub  fn random_match_socket( app_state:AppState)->SocketIoLayer{
    let (layer,io)=SocketIo::builder().with_state(app_state).build_layer();
    io.ns("/random-match",| socket:SocketRef, State(app_state):State<AppState>|{
        let token=socket.req_parts().headers.get("authorization");
        if token.is_none(){
            socket.disconnect().unwrap_or_default();
            return;
        }
        let token=token.unwrap().to_str().unwrap();
        let user=verify_token(token, app_state);
        if user.is_err(){
            socket.disconnect().unwrap_or_default();
            return;
        }
        socket.extensions.insert(user.unwrap());
         socket.on("join_random_match",join_random_match);
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