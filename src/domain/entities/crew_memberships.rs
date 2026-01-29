use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    domain::entities::{brawlers::BrawlerEntity, missions::MissionEntity},
    infrastructure::database::schema::crew_memberships,
};

pub const MAX_CREW_MEMBERSHIPS_PER_MISSION: u32 = 10;

#[derive(Debug, Clone, Queryable, Insertable,Associations, Serialize,Selectable, Deserialize)]
#[diesel(belongs_to(BrawlerEntity, foreign_key = brawler_id))]
#[diesel(belongs_to(MissionEntity, foreign_key = mission_id))]
#[diesel(table_name = crew_memberships)]
pub struct CrewMembershipEntity {
    pub brawler_id: i32,
    pub mission_id: i32,
}   