pub trait ApiMapper<Entity,Presenter,Payload>{
    fn api(entity : Entity) -> Presenter;
    fn entity(payload: Payload) -> Entity;
}