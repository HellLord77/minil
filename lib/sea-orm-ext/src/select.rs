use sea_orm::*;

pub trait SelectExt<E>
where
    E: EntityTrait,
{
    fn find_both_related<R>(self, r: R) -> SelectTwo<E, R>
    where
        R: EntityTrait,
        E: Related<R>;
}

impl<E> SelectExt<E> for Select<E>
where
    E: EntityTrait,
{
    fn find_both_related<R>(self, r: R) -> SelectTwo<E, R>
    where
        R: EntityTrait,
        E: Related<R>,
    {
        self.inner_join(r).select_also(r)
    }
}
