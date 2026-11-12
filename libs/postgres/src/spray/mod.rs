mod spray_complete;
mod spray_task;
use crate::{diesel::prelude::*, schema::*};
#[cfg(test)]
use macros_lib::Mockable;
use postgis_diesel::gps::*;
pub use spray_complete::NestedSprayCompleteInsert;
pub use spray_task::{NestedSprayTaskInsert, SprayTaskInsert};

#[derive(PartialEq, Debug, Clone, Identifiable, Queryable, Insertable, AsChangeset)]
#[cfg_attr(test, derive(Mockable))]
#[diesel(primary_key(task_id))]
#[diesel(table_name = spraytask)]
pub struct SprayTaskPersistent {
    pub task_id: i64,
    pub task_name: String,
    pub customer_name: String,
    pub farm_name: Option<String>,
    pub fruit_group_name: String,
    pub fruit_name: String,
    pub plain_name: String,
    pub plain_size: i64,
    pub geom: Polygon,
}

#[derive(
    PartialEq, Debug, Clone, Identifiable, Queryable, LabelledGeneric, Associations, AsChangeset,
)]
#[diesel(belongs_to(SprayTaskPersistent, foreign_key = task_id))]
#[diesel(belongs_to(SprayCompletePersistent, foreign_key = complete_id))]
#[diesel(table_name = spray_plantprotection)]
pub struct SprayPlantProtectionPersistent {
    pub id: i64,
    pub task_id: Option<i64>,
    pub complete_id: Option<i64>,
    pub name: String,
    pub amount: i64,
    pub unit: String,
}

#[derive(LabelledGeneric, Insertable, Debug, Default, Clone, PartialEq)]
#[cfg_attr(test, derive(Mockable))]
#[diesel(table_name = spray_plantprotection)]
pub struct SprayPlantProtectionInsert {
    #[cfg_attr(test, mockable_const(None))]
    pub task_id: Option<i64>,
    #[cfg_attr(test, mockable_const(None))]
    pub complete_id: Option<i64>,
    pub name: String,
    pub amount: i64,
    pub unit: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct NestedSprayTask {
    pub spray: SprayTaskPersistent,
    pub protections: Vec<SprayPlantProtectionPersistent>,
}

#[derive(PartialEq, Debug, Clone, Identifiable, Queryable, Insertable, AsChangeset)]
#[cfg_attr(test, derive(Mockable))]
#[diesel(primary_key(task_id))]
#[diesel(table_name = spraycomplete)]
pub struct SprayCompletePersistent {
    pub task_id: i64,
    pub task_name: String,
    pub customer_name: String,
    pub farm_name: Option<String>,
    pub fruit_group_name: String,
    pub fruit_name: String,
    pub plain_name: String,
    pub plain_size: i64,
    pub application_total_volume: Option<i64>,
    pub total_area: Option<i64>,
    pub effective_total_distance: Option<i64>,
    pub effective_total_time: Option<i64>,
    pub geom: Polygon,
}

#[derive(PartialEq, Debug, Clone)]
pub struct NestedSprayComplete {
    pub spray: SprayCompletePersistent,
    pub protections: Vec<SprayPlantProtectionPersistent>,
}
