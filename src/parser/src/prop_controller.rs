use crate::collect_data::PropType;
use crate::maps::BUTTONMAP;
use crate::maps::TYPEHM;
use crate::parser_thread_settings::SpecialIDs;
use crate::sendtables::Field;
use crate::sendtables::Serializer;
use ahash::AHashMap;

const WEAPON_NAME_ID: u32 = 1;
const YAW_ID: u32 = 2;
const PITCH_ID: u32 = 3;
const TICK_ID: u32 = 4;
const STEAMID_ID: u32 = 5;
const NAME_ID: u32 = 6;
const PLAYER_X_ID: u32 = 7;
const PLAYER_Y_ID: u32 = 8;
const PLAYER_Z_ID: u32 = 9;

const BUTTONS_BASEID: u32 = 100000;
const NORMAL_PROP_BASEID: u32 = 1000;

#[derive(Clone, Debug)]
pub struct PropController {
    pub id: u32,
    pub wanted_player_props: Vec<String>,
    pub wanted_prop_ids: Vec<u32>,
    pub prop_infos: Vec<PropInfo>,
    pub prop_name_to_path: AHashMap<String, [i32; 7]>,
    pub path_to_prop_name: AHashMap<[i32; 7], String>,
    pub name_to_id: AHashMap<String, u32>,
    pub id_to_name: AHashMap<u32, String>,
    pub special_ids: SpecialIDs,
    pub wanted_player_og_props: Vec<String>,
    pub real_name_to_og_name: AHashMap<String, String>,
    pub name_to_special_id: AHashMap<String, u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropInfo {
    pub id: u32,
    pub prop_type: Option<PropType>,
    pub prop_name: String,
    pub prop_friendly_name: String,
}

impl PropController {
    pub fn new(
        wanted_player_props: Vec<String>,
        wanted_player_props_og_names: Vec<String>,
        real_name_to_og_name: AHashMap<String, String>,
    ) -> Self {
        PropController {
            id: NORMAL_PROP_BASEID,
            wanted_player_props: wanted_player_props,
            wanted_prop_ids: vec![],
            prop_infos: vec![],
            prop_name_to_path: AHashMap::default(),
            path_to_prop_name: AHashMap::default(),
            name_to_id: AHashMap::default(),
            special_ids: SpecialIDs::new(),
            wanted_player_og_props: wanted_player_props_og_names,
            id_to_name: AHashMap::default(),
            real_name_to_og_name: real_name_to_og_name,
            name_to_special_id: AHashMap::default(),
        }
    }
    pub fn set_custom_propinfos(&mut self) {
        let button_names = BUTTONMAP.keys();
        let mut someid = BUTTONS_BASEID;
        for bn in button_names {
            if self.wanted_player_props.contains(&(bn.to_string())) {
                self.prop_infos.push(PropInfo {
                    id: someid,
                    prop_type: Some(PropType::Button),
                    prop_name: bn.to_string(),
                    prop_friendly_name: bn.to_string(),
                });
                someid += 1;
            }
        }
        if self.wanted_player_props.contains(&("weapon_name".to_string())) {
            self.prop_infos.push(PropInfo {
                id: WEAPON_NAME_ID,
                prop_type: Some(PropType::Custom),
                prop_name: "active_weapon_name".to_string(),
                prop_friendly_name: "active_weapon_name".to_string(),
            });
        }
        if self.wanted_player_props.contains(&("pitch".to_string())) {
            self.prop_infos.push(PropInfo {
                id: PITCH_ID,
                prop_type: Some(PropType::Custom),
                prop_name: "pitch".to_string(),
                prop_friendly_name: "pitch".to_string(),
            });
        }
        if self.wanted_player_props.contains(&("yaw".to_string())) {
            self.prop_infos.push(PropInfo {
                id: YAW_ID,
                prop_type: Some(PropType::Custom),
                prop_name: "yaw".to_string(),
                prop_friendly_name: "yaw".to_string(),
            });
        }
        if self.wanted_player_props.contains(&("X".to_string())) {
            self.prop_infos.push(PropInfo {
                id: PLAYER_X_ID,
                prop_type: Some(PropType::Custom),
                prop_name: "X".to_string(),
                prop_friendly_name: "X".to_string(),
            });
        }
        if self.wanted_player_props.contains(&("Y".to_string())) {
            self.prop_infos.push(PropInfo {
                id: PLAYER_Y_ID,
                prop_type: Some(PropType::Custom),
                prop_name: "Y".to_string(),
                prop_friendly_name: "Y".to_string(),
            });
        }
        if self.wanted_player_props.contains(&("Z".to_string())) {
            self.prop_infos.push(PropInfo {
                id: PLAYER_Z_ID,
                prop_type: Some(PropType::Custom),
                prop_name: "Z".to_string(),
                prop_friendly_name: "Z".to_string(),
            });
        }
        self.prop_infos.push(PropInfo {
            id: TICK_ID,
            prop_type: Some(PropType::Tick),
            prop_name: "tick".to_string(),
            prop_friendly_name: "tick".to_string(),
        });
        self.prop_infos.push(PropInfo {
            id: STEAMID_ID,
            prop_type: Some(PropType::Steamid),
            prop_name: "steamid".to_string(),
            prop_friendly_name: "steamid".to_string(),
        });
        self.prop_infos.push(PropInfo {
            id: NAME_ID,
            prop_type: Some(PropType::Name),
            prop_name: "name".to_string(),
            prop_friendly_name: "name".to_string(),
        });
    }
    pub fn find_prop_name_paths(&mut self, ser: &mut Serializer) {
        self.traverse_fields(&mut ser.fields, ser.name.clone())
    }
    pub fn vec_to_arr(path: &Vec<i32>) -> [i32; 7] {
        let mut arr = [0, 0, 0, 0, 0, 0, 0];
        for (idx, val) in path.iter().enumerate() {
            arr[idx] = *val;
        }
        arr
    }
    fn set_id(&mut self, weap_prop: &str, f: &mut Field) {
        match self.name_to_id.get(weap_prop) {
            // If we already have an id for prop of same name then use that id.
            // Mainly for weapon props. For example CAK47.m_iClip1 and CWeaponSCAR20.m_iClip1
            // are the "same" prop. (they have same path and we want to refer to it with one id not ~20)
            Some(id) => {
                f.prop_id = *id as usize;
                return;
            }
            None => {
                self.name_to_id.insert(weap_prop.to_string(), self.id);
                f.prop_id = self.id as usize;
            }
        }
    }
    fn insert_propinfo(&mut self, weap_prop: &str, f: &mut Field) {
        let prop_type = TYPEHM.get(&weap_prop);
        if self.should_collect(weap_prop) {
            self.prop_infos.push(PropInfo {
                id: f.prop_id as u32,
                prop_type: prop_type.copied(),
                prop_name: weap_prop.to_string(),
                prop_friendly_name: self
                    .real_name_to_og_name
                    .get(&weap_prop.to_string())
                    .unwrap_or(&weap_prop.to_string())
                    .to_string(),
            })
        }
    }
    pub fn handle_prop(&mut self, full_name: &str, f: &mut Field) {
        // CAK47.m_iClip1 => ["CAK47", "m_iClip1"]
        let split_at_dot: Vec<&str> = full_name.split(".").collect();
        let is_weapon_prop =
            (split_at_dot[0].contains("Weapon") || split_at_dot[0].contains("AK")) && !split_at_dot[0].contains("Player");
        let is_projectile_prop = (split_at_dot[0].contains("Projectile") || split_at_dot[0].contains("Grenade"))
            && !split_at_dot[0].contains("Player");
        let is_grenade_or_weapon = is_weapon_prop || is_projectile_prop;

        // Strip first part of name from grenades and weapons.
        // if weapon prop: CAK47.m_iClip1 => m_iClip1
        // if grenade: CSmokeGrenadeProjectile.CBodyComponentBaseAnimGraph.m_cellX => CBodyComponentBaseAnimGraph.m_cellX
        let prop_name = match is_grenade_or_weapon {
            true => split_at_dot[1..].join("."),
            false => full_name.to_string(),
        };
        let prop_already_exists = self.name_to_id.contains_key(&(prop_name).to_string());
        self.set_special_ids(&prop_name, is_grenade_or_weapon);
        self.set_id(&prop_name, f);
        if !prop_already_exists {
            self.insert_propinfo(&prop_name, f);
        }
        if self.should_parse(&prop_name) {
            f.should_parse = true;
        }
        self.id += 1;
    }
    fn should_collect(&self, name: &str) -> bool {
        self.wanted_player_props.contains(&(name.to_string()))
    }
    fn should_parse(&self, name: &str) -> bool {
        if self.wanted_player_props.contains(&"X".to_string())
            || self.wanted_player_props.contains(&"Y".to_string())
            || self.wanted_player_props.contains(&"Z".to_string())
        {
            if name.contains("cell") || name.contains("m_vec") {
                return true;
            }
        }
        let always_parse = vec![
            "m_nOwnerId",
            "m_iItemDefinitionIndex",
            "CCSPlayerPawn.CCSPlayer_MovementServices.m_nButtonDownMaskPrev",
            "CCSPlayerPawn.CCSPlayer_WeaponServices.m_hActiveWeapon",
            "CCSPlayerPawn.m_iTeamNum",
            "CBasePlayerWeapon.m_nOwnerId",
            "CCSPlayerPawn.CCSPlayer_WeaponServices.m_hActiveWeapon",
        ];
        if self.wanted_player_props.contains(&("yaw").to_string())
            || self.wanted_player_props.contains(&("pitch").to_string()) && name == "CCSPlayerPawn.m_angEyeAngles"
        {
            return true;
        }
        if always_parse.contains(&name) {
            return true;
        }
        match TYPEHM.get(name) {
            Some(PropType::Weapon) => return true,
            _ => {}
        };
        if name.contains("CCSTeam.m_iTeamNum")
            || name.contains("CCSPlayerPawn.m_iTeamNum")
            || name.contains("CCSPlayerController.m_iTeamNum")
            || name.contains("CCSPlayerController.m_iszPlayerName")
            || name.contains("CCSPlayerController.m_steamID")
            || name.contains("CCSPlayerController.m_hPlayerPawn")
            || name.contains("CCSPlayerController.m_bPawnIsAlive")
            || name.contains("m_hActiveWeapon")
        {
            return true;
        }
        if self.wanted_player_props.contains(&name.to_owned()) {
            return true;
        }
        false
    }
    fn set_special_ids(&mut self, name: &str, is_grenade_or_weapon: bool) {
        if is_grenade_or_weapon {
            match name {
                "m_nOwnerId" => self.special_ids.grenade_owner_id = Some(self.id),
                "CBodyComponentBaseAnimGraph.m_vecX" => self.special_ids.m_vec_x_grenade = Some(self.id),
                "CBodyComponentBaseAnimGraph.m_vecY" => self.special_ids.m_vec_y_grenade = Some(self.id),
                "CBodyComponentBaseAnimGraph.m_vecZ" => self.special_ids.m_vec_z_grenade = Some(self.id),
                "CBodyComponentBaseAnimGraph.m_cellX" => self.special_ids.m_cell_x_grenade = Some(self.id),
                "CBodyComponentBaseAnimGraph.m_cellY" => self.special_ids.m_cell_y_grenade = Some(self.id),
                "CBodyComponentBaseAnimGraph.m_cellZ" => self.special_ids.m_cell_z_grenade = Some(self.id),
                "m_iItemDefinitionIndex" => self.special_ids.item_def = Some(self.id),
                "CCSPlayerPawn.CCSPlayer_MovementServices.m_nButtonDownMaskPrev" => self.special_ids.buttons = Some(self.id),
                "CCSPlayerPawn.CCSPlayer_WeaponServices.m_hActiveWeapon" => self.special_ids.active_weapon = Some(self.id),
                _ => {}
            };
        } else {
            match name {
                "CCSTeam.m_iTeamNum" => self.special_ids.team_team_num = Some(self.id),
                "CCSPlayerPawn.m_iTeamNum" => self.special_ids.player_team_pointer = Some(self.id),
                "CBasePlayerWeapon.m_nOwnerId" => self.special_ids.weapon_owner_pointer = Some(self.id),
                "CCSPlayerController.m_iTeamNum" => self.special_ids.teamnum = Some(self.id),
                "CCSPlayerController.m_iszPlayerName" => self.special_ids.player_name = Some(self.id),
                "CCSPlayerController.m_steamID" => self.special_ids.steamid = Some(self.id),
                "CCSPlayerController.m_hPlayerPawn" => self.special_ids.player_pawn = Some(self.id),
                "CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_cellX" => self.special_ids.cell_x_player = Some(self.id),
                "CCSPlayerPawn.CCSPlayer_MovementServices.m_nButtonDownMaskPrev" => self.special_ids.buttons = Some(self.id),
                "CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_vecX" => self.special_ids.cell_x_offset_player = Some(self.id),
                "CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_cellY" => self.special_ids.cell_y_player = Some(self.id),
                "CCSPlayerPawn.m_angEyeAngles" => self.special_ids.eye_angles = Some(self.id),
                "CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_vecY" => self.special_ids.cell_y_offset_player = Some(self.id),
                "CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_cellZ" => self.special_ids.cell_z_player = Some(self.id),
                "CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_vecZ" => self.special_ids.cell_z_offset_player = Some(self.id),
                "CCSPlayerPawn.CCSPlayer_WeaponServices.m_hActiveWeapon" => self.special_ids.active_weapon = Some(self.id),
                _ => {}
            };
        }
    }
    fn traverse_fields(&mut self, fields: &mut Vec<Field>, ser_name: String) {
        for f in fields {
            if let Some(ser) = &mut f.serializer {
                self.traverse_fields(&mut ser.fields, ser_name.clone() + "." + &ser.name)
            } else {
                let full_name = ser_name.clone() + "." + &f.var_name;
                self.handle_prop(&full_name, f);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PropController;
    use crate::collect_data::PropType;
    use crate::prop_controller::{PropInfo, NORMAL_PROP_BASEID, TICK_ID, YAW_ID};
    use crate::prop_controller::{BUTTONS_BASEID, PITCH_ID};
    use crate::prop_controller::{STEAMID_ID, WEAPON_NAME_ID};
    use crate::sendtables::Decoder::BaseDecoder;
    use crate::sendtables::FieldModel::FieldModelNOTSET;
    use crate::sendtables::FieldType;
    use crate::sendtables::{Field, Serializer};
    use ahash::AHashMap;

    pub fn gen_default_field() -> Field {
        Field {
            var_name: "m_nRandomSeedOffset".to_string(),
            var_type: "int32".to_string(),
            send_node: "m_animationController.m_animGraphNetworkedVars".to_string(),
            serializer_name: None,
            encoder: "".to_string(),
            encode_flags: 0,
            bitcount: 0,
            low_value: 0.0,
            high_value: 0.0,
            model: FieldModelNOTSET,
            field_type: FieldType {
                base_type: "int32".to_string(),
                generic_type: None,
                pointer: false,
                count: 0,
            },
            serializer: None,
            decoder: BaseDecoder,
            base_decoder: None,
            child_decoder: None,
            should_parse: false,
            prop_id: 0,
            is_controller_prop: false,
            controller_prop: None,
            idx: 0,
        }
    }
    #[test]
    pub fn test_traverse() {
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        let mut nested = gen_default_field();
        nested.serializer = Some(Serializer {
            name: "inner".to_string(),
            fields: vec![gen_default_field(), gen_default_field(), gen_default_field()],
        });
        let mut fields = vec![nested, gen_default_field(), gen_default_field(), gen_default_field()];
        pc.traverse_fields(&mut fields, "test_name".to_string());
    }
    #[test]
    pub fn test_prop_name_paths() {
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        let mut s = Serializer {
            name: "inner".to_string(),
            fields: vec![gen_default_field(), gen_default_field(), gen_default_field()],
        };
        pc.find_prop_name_paths(&mut s);
    }
    #[test]
    pub fn test_vec_to_arr_basic() {
        let arr = PropController::vec_to_arr(&vec![1, 2, 3]);
        assert_eq!(arr, [1, 2, 3, 0, 0, 0, 0]);
    }
    #[test]
    pub fn test_vec_to_arr_zero_middle() {
        let arr = PropController::vec_to_arr(&vec![1, 2, 0, 3]);
        assert_eq!(arr, [1, 2, 0, 3, 0, 0, 0]);
    }
    #[test]
    pub fn test_smoke_owner_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("SmokeGrenadeProjectile.m_nOwnerId", &mut f);
        assert!(pc.special_ids.grenade_owner_id.is_some())
    }

    #[test]
    pub fn test_smoke_owner_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("X", &mut f);
        assert!(pc.special_ids.grenade_owner_id.is_none())
    }
    #[test]
    pub fn test_custom_propinfos_weapon_name() {
        let mut pc = PropController::new(vec!["weapon_name".to_string()], vec![], AHashMap::default());
        pc.set_custom_propinfos();
        let pi = pc.prop_infos[0].clone();
        assert_eq!(
            pi,
            PropInfo {
                id: WEAPON_NAME_ID,
                prop_type: Some(PropType::Custom),
                prop_name: "active_weapon_name".to_string(),
                prop_friendly_name: "active_weapon_name".to_string()
            }
        );
    }
    #[test]
    pub fn test_custom_propinfos_a() {
        let mut pc = PropController::new(vec!["A".to_string()], vec![], AHashMap::default());
        pc.set_custom_propinfos();
        let pi = pc.prop_infos[0].clone();
        assert_eq!(
            pi,
            PropInfo {
                id: BUTTONS_BASEID,
                prop_type: Some(PropType::Button),
                prop_name: "A".to_string(),
                prop_friendly_name: "A".to_string()
            }
        );
    }
    #[test]
    pub fn test_custom_propinfos_steamid() {
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.set_custom_propinfos();
        let pi = pc.prop_infos[1].clone();
        assert_eq!(
            pi,
            PropInfo {
                id: STEAMID_ID,
                prop_type: Some(PropType::Steamid),
                prop_name: "steamid".to_string(),
                prop_friendly_name: "steamid".to_string()
            }
        );
    }
    #[test]
    pub fn test_custom_propinfos_tick() {
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.set_custom_propinfos();
        let pi = pc.prop_infos[0].clone();
        assert_eq!(
            pi,
            PropInfo {
                id: TICK_ID,
                prop_type: Some(PropType::Tick),
                prop_name: "tick".to_string(),
                prop_friendly_name: "tick".to_string()
            }
        );
    }
    #[test]
    pub fn test_custom_propinfos_pitch() {
        let mut pc = PropController::new(vec!["pitch".to_string()], vec![], AHashMap::default());
        pc.set_custom_propinfos();
        let pi = pc.prop_infos[0].clone();
        assert_eq!(
            pi,
            PropInfo {
                id: PITCH_ID,
                prop_type: Some(PropType::Custom),
                prop_name: "pitch".to_string(),
                prop_friendly_name: "pitch".to_string()
            }
        );
    }
    #[test]
    pub fn test_custom_propinfos_yaw() {
        let mut pc = PropController::new(vec!["yaw".to_string()], vec![], AHashMap::default());
        pc.set_custom_propinfos();
        let pi = pc.prop_infos[0].clone();
        assert_eq!(
            pi,
            PropInfo {
                id: YAW_ID,
                prop_type: Some(PropType::Custom),
                prop_name: "yaw".to_string(),
                prop_friendly_name: "yaw".to_string()
            }
        );
    }
    #[test]
    pub fn test_special_ids_teamnum_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSTeam.m_iTeamNum", &mut f);
        assert!(pc.special_ids.team_team_num.is_some());
    }
    #[test]
    pub fn test_special_ids_teamnum_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_cellY", &mut f);
        assert!(pc.special_ids.team_team_num.is_none());
    }
    #[test]
    pub fn test_special_ids_player_cell_x_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_cellX", &mut f);
        assert!(pc.special_ids.cell_x_player.is_some());
    }
    #[test]
    pub fn test_special_ids_player_cell_x_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSTeam.m_iTeamNum", &mut f);
        assert!(pc.special_ids.cell_x_player.is_none());
    }
    #[test]
    pub fn test_special_ids_player_cell_y_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_cellY", &mut f);
        assert!(pc.special_ids.cell_y_player.is_some());
    }
    #[test]
    pub fn test_special_ids_player_cell_y_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSTeam.m_iTeamNum", &mut f);
        assert!(pc.special_ids.cell_y_player.is_none());
    }
    #[test]
    pub fn test_special_ids_player_cell_z_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_cellZ", &mut f);
        assert!(pc.special_ids.cell_z_player.is_some());
    }
    #[test]
    pub fn test_special_ids_player_cell_z_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSTeam.m_iTeamNum", &mut f);
        assert!(pc.special_ids.cell_z_player.is_none());
    }
    #[test]
    pub fn test_special_ids_player_offset_x_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_vecX", &mut f);
        assert!(pc.special_ids.cell_x_offset_player.is_some());
    }
    #[test]
    pub fn test_special_ids_player_offset_x_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSTeam.m_iTeamNum", &mut f);
        assert!(pc.special_ids.cell_x_offset_player.is_none());
    }
    #[test]
    pub fn test_special_ids_player_offset_y_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_vecY", &mut f);
        assert!(pc.special_ids.cell_y_offset_player.is_some());
    }
    #[test]
    pub fn test_special_ids_player_offset_y_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSTeam.m_iTeamNum", &mut f);
        assert!(pc.special_ids.cell_y_offset_player.is_none());
    }
    #[test]
    pub fn test_special_ids_player_offset_z_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_vecZ", &mut f);
        assert!(pc.special_ids.cell_z_offset_player.is_some());
    }
    #[test]
    pub fn test_special_ids_player_offset_z_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSTeam.m_iTeamNum", &mut f);
        assert!(pc.special_ids.cell_z_offset_player.is_none());
    }
    #[test]
    pub fn test_special_ids_grenade_cell_x_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CSmokeGrenadeProjectile.CBodyComponentBaseAnimGraph.m_cellX", &mut f);
        assert!(pc.special_ids.m_cell_x_grenade.is_some());
    }
    #[test]
    pub fn test_special_ids_grenade_cell_x_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSTeam.m_iTeamNum", &mut f);
        assert!(pc.special_ids.m_cell_x_grenade.is_none());
    }
    #[test]
    pub fn test_special_ids_grenade_cell_y_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CSmokeGrenadeProjectile.CBodyComponentBaseAnimGraph.m_cellY", &mut f);
        assert!(pc.special_ids.m_cell_y_grenade.is_some());
    }
    #[test]
    pub fn test_special_ids_grenade_cell_y_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSTeam.m_iTeamNum", &mut f);
        assert!(pc.special_ids.m_cell_y_grenade.is_none());
    }
    #[test]
    pub fn test_special_ids_grenade_cell_z_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CSmokeGrenadeProjectile.CBodyComponentBaseAnimGraph.m_cellZ", &mut f);
        assert!(pc.special_ids.m_cell_z_grenade.is_some());
    }
    #[test]
    pub fn test_special_ids_grenade_cell_z_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSTeam.m_iTeamNum", &mut f);
        assert!(pc.special_ids.m_cell_z_grenade.is_none());
    }
    #[test]
    pub fn test_special_ids_item_def_idx_ak_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CAK47.m_iItemDefinitionIndex", &mut f);
        assert!(pc.special_ids.item_def.is_some());
    }
    #[test]
    pub fn test_special_ids_item_def_idx_normal_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CWeaponNOVA.m_iItemDefinitionIndex", &mut f);
        assert!(pc.special_ids.item_def.is_some());
    }
    #[test]
    pub fn test_special_ids_item_def_idx_ak_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("X", &mut f);
        assert!(pc.special_ids.item_def.is_none());
    }
    #[test]
    pub fn test_special_ids_item_def_idx_normal_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("Y", &mut f);
        assert!(pc.special_ids.item_def.is_none());
    }
    #[test]
    pub fn test_special_ids_eyeangles_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.m_angEyeAngles", &mut f);
        assert!(pc.special_ids.eye_angles.is_some());
    }
    #[test]
    pub fn test_special_ids_eyeangles_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("Z", &mut f);
        assert!(pc.special_ids.eye_angles.is_none());
    }
    #[test]
    pub fn test_special_ids_playercontroller_team() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerController.m_iTeamNum", &mut f);
        assert!(pc.special_ids.teamnum.is_some());
    }
    #[test]
    pub fn test_special_ids_playercontroller_team_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("Z", &mut f);
        assert!(pc.special_ids.eye_angles.is_none());
    }
    #[test]
    pub fn test_special_ids_playercontroller_name() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerController.m_iszPlayerName", &mut f);
        assert!(pc.special_ids.player_name.is_some());
    }
    #[test]
    pub fn test_special_ids_playercontroller_name_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("Z", &mut f);
        assert!(pc.special_ids.player_name.is_none());
    }
    #[test]
    pub fn test_special_ids_playercontroller_steamid() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerController.m_steamID", &mut f);
        assert!(pc.special_ids.steamid.is_some());
    }
    #[test]
    pub fn test_special_ids_playercontroller_steamid_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("Z", &mut f);
        assert!(pc.special_ids.steamid.is_none());
    }
    #[test]
    pub fn test_special_ids_playercontroller_player_pawn() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerController.m_hPlayerPawn", &mut f);
        assert!(pc.special_ids.player_pawn.is_some());
    }
    #[test]
    pub fn test_special_ids_playercontroller_player_pawn_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("X", &mut f);
        assert!(pc.special_ids.player_pawn.is_none());
    }
    #[test]
    pub fn test_special_ids_weapon_owner_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CBasePlayerWeapon.m_nOwnerId", &mut f);
        assert!(pc.special_ids.weapon_owner_pointer.is_some());
    }
    #[test]
    pub fn test_special_ids_weapon_owner_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("X", &mut f);
        assert!(pc.special_ids.weapon_owner_pointer.is_none());
    }
    #[test]
    pub fn test_special_ids_active_weapon_handle_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.CCSPlayer_WeaponServices.m_hActiveWeapon", &mut f);
        assert!(pc.special_ids.active_weapon.is_some());
    }
    #[test]
    pub fn test_special_ids_active_weapon_handle_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("X", &mut f);
        assert!(pc.special_ids.weapon_owner_pointer.is_none());
    }
    #[test]
    pub fn test_weapon_prop_ammo_normal() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec!["m_iClip1".to_string()], vec![], AHashMap::default());
        pc.handle_prop("CWeaponSCAR20.m_iClip1", &mut f);
        let correct = PropInfo {
            id: NORMAL_PROP_BASEID,
            prop_type: Some(PropType::Weapon),
            prop_friendly_name: "m_iClip1".to_string(),
            prop_name: "m_iClip1".to_string(),
        };
        assert_eq!(pc.prop_infos[0], correct);
    }
    #[test]
    pub fn test_weapon_prop_ammo_normal_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec!["m_iClip1".to_string()], vec![], AHashMap::default());
        pc.handle_prop("CWeaponCSBase.m_iItemDefinitionIndex", &mut f);
        assert_eq!(pc.prop_infos.len(), 0);
    }
    #[test]
    pub fn test_weapon_prop_ammo_ak() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec!["m_iClip1".to_string()], vec![], AHashMap::default());
        pc.handle_prop("CAK47.m_iClip1", &mut f);
        let correct = PropInfo {
            id: NORMAL_PROP_BASEID,
            prop_type: Some(PropType::Weapon),
            prop_friendly_name: "m_iClip1".to_string(),
            prop_name: "m_iClip1".to_string(),
        };
        assert_eq!(pc.prop_infos[0], correct);
    }
    #[test]
    pub fn test_weapon_prop_ammo_ak_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec!["X".to_string()], vec![], AHashMap::default());
        pc.handle_prop("CWeaponCSBase.m_iItemDefinitionIndex", &mut f);
        assert_eq!(pc.prop_infos.len(), 0);
    }
    #[test]
    pub fn test_normal_prop_health() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec!["CCSPlayerPawn.m_iHealth".to_string()], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.m_iHealth", &mut f);
        let correct = PropInfo {
            id: NORMAL_PROP_BASEID,
            prop_type: Some(PropType::Player),
            prop_friendly_name: "CCSPlayerPawn.m_iHealth".to_string(),
            prop_name: "CCSPlayerPawn.m_iHealth".to_string(),
        };
        assert_eq!(pc.prop_infos[0], correct);
    }
    #[test]
    pub fn test_normal_prop_not_set() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.m_iHealth", &mut f);
        assert_eq!(pc.prop_infos.len(), 0);
    }
    #[test]
    pub fn test_weapon_prop_duplicate_name_ak_normal() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec!["m_iClip1".to_string()], vec![], AHashMap::default());
        pc.handle_prop("CAK47.m_iClip1", &mut f);
        pc.handle_prop("CWeaponSCAR20.m_iClip1", &mut f);

        let correct = PropInfo {
            id: NORMAL_PROP_BASEID,
            prop_type: Some(PropType::Weapon),
            prop_friendly_name: "m_iClip1".to_string(),
            prop_name: "m_iClip1".to_string(),
        };
        assert_eq!(pc.prop_infos.len(), 1);
        assert_eq!(pc.prop_infos[0], correct);
        assert_eq!(pc.id, NORMAL_PROP_BASEID + 2);
    }
    #[test]
    pub fn test_weapon_prop_duplicate_name_normal_normal() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec!["m_iClip1".to_string()], vec![], AHashMap::default());
        pc.handle_prop("CWeaponNOVA.m_iClip1", &mut f);
        pc.handle_prop("CWeaponSCAR20.m_iClip1", &mut f);

        let correct = PropInfo {
            id: NORMAL_PROP_BASEID,
            prop_type: Some(PropType::Weapon),
            prop_friendly_name: "m_iClip1".to_string(),
            prop_name: "m_iClip1".to_string(),
        };
        assert_eq!(pc.prop_infos[0], correct);
    }
    #[test]
    pub fn test_normal_prop_should_parse() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec!["CCSPlayerPawn.m_iHealth".to_string()], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.m_iHealth", &mut f);
        assert_eq!(f.should_parse, true);
    }
    #[test]
    pub fn test_normal_prop_should_not_parse() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.m_iHealth", &mut f);
        assert_eq!(f.should_parse, false);
    }
    #[test]
    pub fn test_special_prop_should_parse() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec!["Y".to_string()], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_cellY", &mut f);
        assert_eq!(f.should_parse, true);
    }
    #[test]
    pub fn test_special_prop_should_not_parse() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec!["health".to_string()], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_cellY", &mut f);
        assert_eq!(f.should_parse, false);
    }
    #[test]
    pub fn test_yaw() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec!["yaw".to_string()], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.m_angEyeAngles", &mut f);
        assert_eq!(f.should_parse, true);
    }
    #[test]
    pub fn test_pitch() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec!["pitch".to_string()], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.m_angEyeAngles", &mut f);
        assert_eq!(f.should_parse, true);
        assert_eq!(f.prop_id, NORMAL_PROP_BASEID as usize);
    }
    #[test]
    pub fn test_pitch_dont_parse_eyeang() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.m_angEyeAngles", &mut f);
        assert_eq!(f.should_parse, false);
        assert_eq!(f.prop_id, NORMAL_PROP_BASEID as usize);
    }
    #[test]
    pub fn test_yaw_dont_parse_eyeang() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec![], vec![], AHashMap::default());
        pc.handle_prop("CCSPlayerPawn.m_angEyeAngles", &mut f);
        assert_eq!(f.should_parse, false);
        assert_eq!(f.prop_id, NORMAL_PROP_BASEID as usize);
    }
    /*
    #[test]
    pub fn test_player_x_propinfo() {
        let mut f = gen_default_field();
        let mut pc = PropController::new(vec!["X".to_string()], vec![], AHashMap::default());
        pc.handle_prop("X", &mut f);

        let correct = PropInfo {
            id: PLAYER_X_ID,
            prop_type: Some(PropType::Custom),
            prop_friendly_name: "X".to_string(),
            prop_name: "X".to_string(),
        };
        assert_eq!(pc.prop_infos.len(), 1);
        assert_eq!(pc.prop_infos[0], correct);
    }
    */
}
