use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::{domain::entities::{
    missions::NewMission, missions::UpdateMission
},
    domain::value_objects::mission_statuses::MissionStatuses,

};


#[derive(Debug, Clone, Serialize, Deserialize,PartialEq)]
pub struct MissionModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub chief_id: i32,
    pub crew_count: i64,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}


#[derive(Debug, Clone, Serialize, Deserialize,PartialEq)]
pub struct NewMissionModel {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}

impl NewMissionModel {
    pub fn to_entity(&self, chief_id: i32) -> NewMission {
        NewMission {
            name: self.name.clone(),
            description: self.description.clone().unwrap_or_default(),
            status: MissionStatuses::Open.to_string(),
            chief_id,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize,PartialEq)]
pub struct UpdateMissionModel {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

impl UpdateMissionModel {
    pub fn to_entity(&self, chief_id: i32) -> UpdateMission {
        UpdateMission {
            name: self.name.clone(),
            description: self.description.clone(),
            status: self.status.clone(),
            chief_id,
        }
    }
}