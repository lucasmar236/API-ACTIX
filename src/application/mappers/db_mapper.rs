pub trait DbMapper<Entity,DbModel>{
    fn db(entity: Entity) -> DbModel;
    fn entity(model: DbModel) -> Entity;
}