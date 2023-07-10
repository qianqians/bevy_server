// Autogenerated by Thrift Compiler (0.18.1)
// DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![allow(clippy::too_many_arguments, clippy::type_complexity, clippy::vec_box, clippy::wrong_self_convention)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::{From, TryFrom};
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use thrift::OrderedFloat;
use thrift::{ApplicationError, ApplicationErrorKind, ProtocolError, ProtocolErrorKind, TThriftClient};
use thrift::protocol::{TFieldIdentifier, TListIdentifier, TMapIdentifier, TMessageIdentifier, TMessageType, TInputProtocol, TOutputProtocol, TSerializable, TSetIdentifier, TStructIdentifier, TType};
use thrift::protocol::field_id;
use thrift::protocol::verify_expected_message_type;
use thrift::protocol::verify_expected_sequence_number;
use thrift::protocol::verify_expected_service_call;
use thrift::protocol::verify_required_field_exists;
use thrift::server::TProcessor;

use crate::common;

//
// RegHub
//

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RegHub {
  pub hub_name: Option<String>,
  pub hub_type: Option<String>,
}

impl RegHub {
  pub fn new<F1, F2>(hub_name: F1, hub_type: F2) -> RegHub where F1: Into<Option<String>>, F2: Into<Option<String>> {
    RegHub {
      hub_name: hub_name.into(),
      hub_type: hub_type.into(),
    }
  }
}

impl TSerializable for RegHub {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<RegHub> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<String> = Some("".to_owned());
    let mut f_2: Option<String> = Some("".to_owned());
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_string()?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_string()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = RegHub {
      hub_name: f_1,
      hub_type: f_2,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("reg_hub");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.hub_name {
      o_prot.write_field_begin(&TFieldIdentifier::new("hub_name", TType::String, 1))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.hub_type {
      o_prot.write_field_begin(&TFieldIdentifier::new("hub_type", TType::String, 2))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// HubCallClientCreateRemoteEntity
//

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HubCallClientCreateRemoteEntity {
  pub conn_id: Option<Vec<String>>,
  pub main_conn_id: Option<String>,
  pub entity_id: Option<String>,
  pub entity_type: Option<String>,
  pub argvs: Option<Vec<u8>>,
}

impl HubCallClientCreateRemoteEntity {
  pub fn new<F1, F2, F3, F4, F5>(conn_id: F1, main_conn_id: F2, entity_id: F3, entity_type: F4, argvs: F5) -> HubCallClientCreateRemoteEntity where F1: Into<Option<Vec<String>>>, F2: Into<Option<String>>, F3: Into<Option<String>>, F4: Into<Option<String>>, F5: Into<Option<Vec<u8>>> {
    HubCallClientCreateRemoteEntity {
      conn_id: conn_id.into(),
      main_conn_id: main_conn_id.into(),
      entity_id: entity_id.into(),
      entity_type: entity_type.into(),
      argvs: argvs.into(),
    }
  }
}

impl TSerializable for HubCallClientCreateRemoteEntity {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<HubCallClientCreateRemoteEntity> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Vec<String>> = Some(Vec::new());
    let mut f_2: Option<String> = Some("".to_owned());
    let mut f_3: Option<String> = Some("".to_owned());
    let mut f_4: Option<String> = Some("".to_owned());
    let mut f_5: Option<Vec<u8>> = Some(Vec::new());
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<String> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_0 = i_prot.read_string()?;
            val.push(list_elem_0);
          }
          i_prot.read_list_end()?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_string()?;
          f_2 = Some(val);
        },
        3 => {
          let val = i_prot.read_string()?;
          f_3 = Some(val);
        },
        4 => {
          let val = i_prot.read_string()?;
          f_4 = Some(val);
        },
        5 => {
          let val = i_prot.read_bytes()?;
          f_5 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = HubCallClientCreateRemoteEntity {
      conn_id: f_1,
      main_conn_id: f_2,
      entity_id: f_3,
      entity_type: f_4,
      argvs: f_5,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("hub_call_client_create_remote_entity");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.conn_id {
      o_prot.write_field_begin(&TFieldIdentifier::new("conn_id", TType::List, 1))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::String, fld_var.len() as i32))?;
      for e in fld_var {
        o_prot.write_string(e)?;
      }
      o_prot.write_list_end()?;
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.main_conn_id {
      o_prot.write_field_begin(&TFieldIdentifier::new("main_conn_id", TType::String, 2))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.entity_id {
      o_prot.write_field_begin(&TFieldIdentifier::new("entity_id", TType::String, 3))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.entity_type {
      o_prot.write_field_begin(&TFieldIdentifier::new("entity_type", TType::String, 4))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.argvs {
      o_prot.write_field_begin(&TFieldIdentifier::new("argvs", TType::String, 5))?;
      o_prot.write_bytes(fld_var)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// HubCallClientDeleteRemoteEntity
//

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HubCallClientDeleteRemoteEntity {
  pub entity_id: Option<String>,
}

impl HubCallClientDeleteRemoteEntity {
  pub fn new<F1>(entity_id: F1) -> HubCallClientDeleteRemoteEntity where F1: Into<Option<String>> {
    HubCallClientDeleteRemoteEntity {
      entity_id: entity_id.into(),
    }
  }
}

impl TSerializable for HubCallClientDeleteRemoteEntity {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<HubCallClientDeleteRemoteEntity> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<String> = Some("".to_owned());
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_string()?;
          f_1 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = HubCallClientDeleteRemoteEntity {
      entity_id: f_1,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("hub_call_client_delete_remote_entity");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.entity_id {
      o_prot.write_field_begin(&TFieldIdentifier::new("entity_id", TType::String, 1))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// HubCallClientRpc
//

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HubCallClientRpc {
  pub message: Option<common::Msg>,
}

impl HubCallClientRpc {
  pub fn new<F1>(message: F1) -> HubCallClientRpc where F1: Into<Option<common::Msg>> {
    HubCallClientRpc {
      message: message.into(),
    }
  }
}

impl TSerializable for HubCallClientRpc {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<HubCallClientRpc> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<common::Msg> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = common::Msg::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = HubCallClientRpc {
      message: f_1,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("hub_call_client_rpc");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.message {
      o_prot.write_field_begin(&TFieldIdentifier::new("message", TType::Struct, 1))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// HubCallClientRsp
//

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HubCallClientRsp {
  pub rsp: Option<common::RpcRsp>,
}

impl HubCallClientRsp {
  pub fn new<F1>(rsp: F1) -> HubCallClientRsp where F1: Into<Option<common::RpcRsp>> {
    HubCallClientRsp {
      rsp: rsp.into(),
    }
  }
}

impl TSerializable for HubCallClientRsp {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<HubCallClientRsp> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<common::RpcRsp> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = common::RpcRsp::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = HubCallClientRsp {
      rsp: f_1,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("hub_call_client_rsp");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.rsp {
      o_prot.write_field_begin(&TFieldIdentifier::new("rsp", TType::Struct, 1))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// HubCallClientErr
//

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HubCallClientErr {
  pub err: Option<common::RpcErr>,
}

impl HubCallClientErr {
  pub fn new<F1>(err: F1) -> HubCallClientErr where F1: Into<Option<common::RpcErr>> {
    HubCallClientErr {
      err: err.into(),
    }
  }
}

impl TSerializable for HubCallClientErr {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<HubCallClientErr> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<common::RpcErr> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = common::RpcErr::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = HubCallClientErr {
      err: f_1,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("hub_call_client_err");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.err {
      o_prot.write_field_begin(&TFieldIdentifier::new("err", TType::Struct, 1))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// HubCallClientNtf
//

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HubCallClientNtf {
  pub message: Option<common::Msg>,
}

impl HubCallClientNtf {
  pub fn new<F1>(message: F1) -> HubCallClientNtf where F1: Into<Option<common::Msg>> {
    HubCallClientNtf {
      message: message.into(),
    }
  }
}

impl TSerializable for HubCallClientNtf {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<HubCallClientNtf> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<common::Msg> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = common::Msg::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = HubCallClientNtf {
      message: f_1,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("hub_call_client_ntf");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.message {
      o_prot.write_field_begin(&TFieldIdentifier::new("message", TType::Struct, 1))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// HubCallClientGroup
//

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HubCallClientGroup {
  pub entity_id: Option<Vec<String>>,
  pub method: Option<String>,
  pub argvs: Option<Vec<u8>>,
  pub is_in_order: Option<bool>,
}

impl HubCallClientGroup {
  pub fn new<F1, F2, F3, F4>(entity_id: F1, method: F2, argvs: F3, is_in_order: F4) -> HubCallClientGroup where F1: Into<Option<Vec<String>>>, F2: Into<Option<String>>, F3: Into<Option<Vec<u8>>>, F4: Into<Option<bool>> {
    HubCallClientGroup {
      entity_id: entity_id.into(),
      method: method.into(),
      argvs: argvs.into(),
      is_in_order: is_in_order.into(),
    }
  }
}

impl TSerializable for HubCallClientGroup {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<HubCallClientGroup> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Vec<String>> = Some(Vec::new());
    let mut f_2: Option<String> = Some("".to_owned());
    let mut f_3: Option<Vec<u8>> = Some(Vec::new());
    let mut f_4: Option<bool> = Some(false);
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<String> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_1 = i_prot.read_string()?;
            val.push(list_elem_1);
          }
          i_prot.read_list_end()?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_string()?;
          f_2 = Some(val);
        },
        3 => {
          let val = i_prot.read_bytes()?;
          f_3 = Some(val);
        },
        4 => {
          let val = i_prot.read_bool()?;
          f_4 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = HubCallClientGroup {
      entity_id: f_1,
      method: f_2,
      argvs: f_3,
      is_in_order: f_4,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("hub_call_client_group");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.entity_id {
      o_prot.write_field_begin(&TFieldIdentifier::new("entity_id", TType::List, 1))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::String, fld_var.len() as i32))?;
      for e in fld_var {
        o_prot.write_string(e)?;
      }
      o_prot.write_list_end()?;
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.method {
      o_prot.write_field_begin(&TFieldIdentifier::new("method", TType::String, 2))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.argvs {
      o_prot.write_field_begin(&TFieldIdentifier::new("argvs", TType::String, 3))?;
      o_prot.write_bytes(fld_var)?;
      o_prot.write_field_end()?
    }
    if let Some(fld_var) = self.is_in_order {
      o_prot.write_field_begin(&TFieldIdentifier::new("is_in_order", TType::Bool, 4))?;
      o_prot.write_bool(fld_var)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// HubCallClientGlobal
//

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HubCallClientGlobal {
  pub method: Option<String>,
  pub argvs: Option<Vec<u8>>,
  pub is_in_order: Option<bool>,
}

impl HubCallClientGlobal {
  pub fn new<F1, F2, F3>(method: F1, argvs: F2, is_in_order: F3) -> HubCallClientGlobal where F1: Into<Option<String>>, F2: Into<Option<Vec<u8>>>, F3: Into<Option<bool>> {
    HubCallClientGlobal {
      method: method.into(),
      argvs: argvs.into(),
      is_in_order: is_in_order.into(),
    }
  }
}

impl TSerializable for HubCallClientGlobal {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<HubCallClientGlobal> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<String> = Some("".to_owned());
    let mut f_2: Option<Vec<u8>> = Some(Vec::new());
    let mut f_3: Option<bool> = Some(false);
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_string()?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_bytes()?;
          f_2 = Some(val);
        },
        3 => {
          let val = i_prot.read_bool()?;
          f_3 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = HubCallClientGlobal {
      method: f_1,
      argvs: f_2,
      is_in_order: f_3,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("hub_call_client_global");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.method {
      o_prot.write_field_begin(&TFieldIdentifier::new("method", TType::String, 1))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.argvs {
      o_prot.write_field_begin(&TFieldIdentifier::new("argvs", TType::String, 2))?;
      o_prot.write_bytes(fld_var)?;
      o_prot.write_field_end()?
    }
    if let Some(fld_var) = self.is_in_order {
      o_prot.write_field_begin(&TFieldIdentifier::new("is_in_order", TType::Bool, 3))?;
      o_prot.write_bool(fld_var)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// HubCallKickOffClient
//

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HubCallKickOffClient {
  pub conn_id: Option<String>,
  pub prompt_info: Option<String>,
}

impl HubCallKickOffClient {
  pub fn new<F1, F2>(conn_id: F1, prompt_info: F2) -> HubCallKickOffClient where F1: Into<Option<String>>, F2: Into<Option<String>> {
    HubCallKickOffClient {
      conn_id: conn_id.into(),
      prompt_info: prompt_info.into(),
    }
  }
}

impl TSerializable for HubCallKickOffClient {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<HubCallKickOffClient> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<String> = Some("".to_owned());
    let mut f_2: Option<String> = Some("".to_owned());
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_string()?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_string()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = HubCallKickOffClient {
      conn_id: f_1,
      prompt_info: f_2,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("hub_call_kick_off_client");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.conn_id {
      o_prot.write_field_begin(&TFieldIdentifier::new("conn_id", TType::String, 1))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.prompt_info {
      o_prot.write_field_begin(&TFieldIdentifier::new("prompt_info", TType::String, 2))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// GateHubService
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GateHubService {
  RegHub(RegHub),
  CreateRemoteEntity(HubCallClientCreateRemoteEntity),
  DeleteRemoteEntity(HubCallClientDeleteRemoteEntity),
  CallRpc(HubCallClientRpc),
  CallRsp(HubCallClientRsp),
  CallErr(HubCallClientErr),
  CallNtf(HubCallClientNtf),
  CallGroup(HubCallClientGroup),
  CallGlobal(HubCallClientGlobal),
  KickOff(HubCallKickOffClient),
}

impl TSerializable for GateHubService {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<GateHubService> {
    let mut ret: Option<GateHubService> = None;
    let mut received_field_count = 0;
    i_prot.read_struct_begin()?;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = RegHub::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GateHubService::RegHub(val));
          }
          received_field_count += 1;
        },
        2 => {
          let val = HubCallClientCreateRemoteEntity::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GateHubService::CreateRemoteEntity(val));
          }
          received_field_count += 1;
        },
        3 => {
          let val = HubCallClientDeleteRemoteEntity::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GateHubService::DeleteRemoteEntity(val));
          }
          received_field_count += 1;
        },
        4 => {
          let val = HubCallClientRpc::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GateHubService::CallRpc(val));
          }
          received_field_count += 1;
        },
        5 => {
          let val = HubCallClientRsp::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GateHubService::CallRsp(val));
          }
          received_field_count += 1;
        },
        6 => {
          let val = HubCallClientErr::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GateHubService::CallErr(val));
          }
          received_field_count += 1;
        },
        7 => {
          let val = HubCallClientNtf::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GateHubService::CallNtf(val));
          }
          received_field_count += 1;
        },
        8 => {
          let val = HubCallClientGroup::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GateHubService::CallGroup(val));
          }
          received_field_count += 1;
        },
        9 => {
          let val = HubCallClientGlobal::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GateHubService::CallGlobal(val));
          }
          received_field_count += 1;
        },
        10 => {
          let val = HubCallKickOffClient::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GateHubService::KickOff(val));
          }
          received_field_count += 1;
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
          received_field_count += 1;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    if received_field_count == 0 {
      Err(
        thrift::Error::Protocol(
          ProtocolError::new(
            ProtocolErrorKind::InvalidData,
            "received empty union from remote GateHubService"
          )
        )
      )
    } else if received_field_count > 1 {
      Err(
        thrift::Error::Protocol(
          ProtocolError::new(
            ProtocolErrorKind::InvalidData,
            "received multiple fields for union from remote GateHubService"
          )
        )
      )
    } else {
      Ok(ret.expect("return value should have been constructed"))
    }
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("gate_hub_service");
    o_prot.write_struct_begin(&struct_ident)?;
    match *self {
      GateHubService::RegHub(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("reg_hub", TType::Struct, 1))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GateHubService::CreateRemoteEntity(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("create_remote_entity", TType::Struct, 2))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GateHubService::DeleteRemoteEntity(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("delete_remote_entity", TType::Struct, 3))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GateHubService::CallRpc(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("call_rpc", TType::Struct, 4))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GateHubService::CallRsp(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("call_rsp", TType::Struct, 5))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GateHubService::CallErr(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("call_err", TType::Struct, 6))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GateHubService::CallNtf(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("call_ntf", TType::Struct, 7))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GateHubService::CallGroup(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("call_group", TType::Struct, 8))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GateHubService::CallGlobal(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("call_global", TType::Struct, 9))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GateHubService::KickOff(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("kick_off", TType::Struct, 10))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// ClientCallHubRpc
//

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ClientCallHubRpc {
  pub message: Option<common::Msg>,
}

impl ClientCallHubRpc {
  pub fn new<F1>(message: F1) -> ClientCallHubRpc where F1: Into<Option<common::Msg>> {
    ClientCallHubRpc {
      message: message.into(),
    }
  }
}

impl TSerializable for ClientCallHubRpc {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<ClientCallHubRpc> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<common::Msg> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = common::Msg::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = ClientCallHubRpc {
      message: f_1,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("client_call_hub_rpc");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.message {
      o_prot.write_field_begin(&TFieldIdentifier::new("message", TType::Struct, 1))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// ClientCallHubRsp
//

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ClientCallHubRsp {
  pub rsp: Option<common::RpcRsp>,
}

impl ClientCallHubRsp {
  pub fn new<F1>(rsp: F1) -> ClientCallHubRsp where F1: Into<Option<common::RpcRsp>> {
    ClientCallHubRsp {
      rsp: rsp.into(),
    }
  }
}

impl TSerializable for ClientCallHubRsp {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<ClientCallHubRsp> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<common::RpcRsp> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = common::RpcRsp::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = ClientCallHubRsp {
      rsp: f_1,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("client_call_hub_rsp");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.rsp {
      o_prot.write_field_begin(&TFieldIdentifier::new("rsp", TType::Struct, 1))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// ClientCallHubErr
//

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ClientCallHubErr {
  pub err: Option<common::RpcErr>,
}

impl ClientCallHubErr {
  pub fn new<F1>(err: F1) -> ClientCallHubErr where F1: Into<Option<common::RpcErr>> {
    ClientCallHubErr {
      err: err.into(),
    }
  }
}

impl TSerializable for ClientCallHubErr {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<ClientCallHubErr> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<common::RpcErr> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = common::RpcErr::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = ClientCallHubErr {
      err: f_1,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("client_call_hub_err");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.err {
      o_prot.write_field_begin(&TFieldIdentifier::new("err", TType::Struct, 1))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// ClientConfirmKickOff
//

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ClientConfirmKickOff {
}

impl ClientConfirmKickOff {
  pub fn new() -> ClientConfirmKickOff {
    ClientConfirmKickOff {}
  }
}

impl TSerializable for ClientConfirmKickOff {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<ClientConfirmKickOff> {
    i_prot.read_struct_begin()?;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      i_prot.skip(field_ident.field_type)?;
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = ClientConfirmKickOff {};
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("client_confirm_kick_off");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// GateClientService
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GateClientService {
  CallRpc(ClientCallHubRpc),
  CallRsp(ClientCallHubRsp),
  CallErr(ClientCallHubRsp),
  ConfirmKickOff(ClientConfirmKickOff),
}

impl TSerializable for GateClientService {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<GateClientService> {
    let mut ret: Option<GateClientService> = None;
    let mut received_field_count = 0;
    i_prot.read_struct_begin()?;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = ClientCallHubRpc::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GateClientService::CallRpc(val));
          }
          received_field_count += 1;
        },
        2 => {
          let val = ClientCallHubRsp::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GateClientService::CallRsp(val));
          }
          received_field_count += 1;
        },
        3 => {
          let val = ClientCallHubRsp::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GateClientService::CallErr(val));
          }
          received_field_count += 1;
        },
        4 => {
          let val = ClientConfirmKickOff::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GateClientService::ConfirmKickOff(val));
          }
          received_field_count += 1;
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
          received_field_count += 1;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    if received_field_count == 0 {
      Err(
        thrift::Error::Protocol(
          ProtocolError::new(
            ProtocolErrorKind::InvalidData,
            "received empty union from remote GateClientService"
          )
        )
      )
    } else if received_field_count > 1 {
      Err(
        thrift::Error::Protocol(
          ProtocolError::new(
            ProtocolErrorKind::InvalidData,
            "received multiple fields for union from remote GateClientService"
          )
        )
      )
    } else {
      Ok(ret.expect("return value should have been constructed"))
    }
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("gate_client_service");
    o_prot.write_struct_begin(&struct_ident)?;
    match *self {
      GateClientService::CallRpc(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("call_rpc", TType::Struct, 1))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GateClientService::CallRsp(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("call_rsp", TType::Struct, 2))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GateClientService::CallErr(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("call_err", TType::Struct, 3))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GateClientService::ConfirmKickOff(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("confirm_kick_off", TType::Struct, 4))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

