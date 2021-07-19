// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

use crate::error::*;
use crate::marker::ContiguousMemory;
use crate::*;
use core::default::Default;

//
// sgx_attributes.h
//
pub type sgx_misc_select_t = uint32_t;

// Enclave Flags Bit Masks
pub const SGX_FLAGS_INITTED: uint64_t         = 0x0000_0000_0000_0001;    //If set, then the enclave is initialized
pub const SGX_FLAGS_DEBUG: uint64_t           = 0x0000_0000_0000_0002;    //If set, then the enclave is debug
pub const SGX_FLAGS_MODE64BIT: uint64_t       = 0x0000_0000_0000_0004;    //If set, then the enclave is 64 bit
pub const SGX_FLAGS_PROVISION_KEY: uint64_t   = 0x0000_0000_0000_0010;    //If set, then the enclave has access to provision key
pub const SGX_FLAGS_EINITTOKEN_KEY: uint64_t  = 0x0000_0000_0000_0020;    //If set, then the enclave has access to EINITTOKEN key
pub const SGX_FLAGS_KSS: uint64_t             = 0x0000_0000_0000_0080;    //If set enclave uses KSS
pub const SGX_FLAGS_RESERVED: uint64_t        = !(SGX_FLAGS_INITTED
                                                | SGX_FLAGS_DEBUG
                                                | SGX_FLAGS_MODE64BIT
                                                | SGX_FLAGS_PROVISION_KEY
                                                | SGX_FLAGS_EINITTOKEN_KEY
                                                | SGX_FLAGS_KSS);

// XSAVE Feature Request Mask
pub const SGX_XFRM_LEGACY: uint64_t           = 0x0000_0000_0000_0003;  //Legacy XFRM
pub const SGX_XFRM_AVX: uint64_t              = 0x0000_0000_0000_0006;  // AVX
pub const SGX_XFRM_AVX512: uint64_t           = 0x0000_0000_0000_00E6;  // AVX-512 - not supported
pub const SGX_XFRM_MPX: uint64_t              = 0x0000_0000_0000_0018;  // MPX - not supported

pub const SGX_XFRM_RESERVED: uint64_t         = !(SGX_XFRM_LEGACY | SGX_XFRM_AVX);

impl_struct! {
    pub struct sgx_attributes_t {
        pub flags: uint64_t,
        pub xfrm: uint64_t,
    }

    pub struct sgx_misc_attribute_t {
        pub secs_attr: sgx_attributes_t,
        pub misc_select: sgx_misc_select_t,
    }
}

//
// tseal_migration_attr.h
//
pub const FLAGS_NON_SECURITY_BITS: uint64_t   = 0x00FF_FFFF_FFFF_FFC0
                                                | SGX_FLAGS_MODE64BIT
                                                | SGX_FLAGS_PROVISION_KEY
                                                | SGX_FLAGS_EINITTOKEN_KEY;
pub const TSEAL_DEFAULT_FLAGSMASK: uint64_t   = !FLAGS_NON_SECURITY_BITS;
pub const FLAGS_SECURITY_BITS_RESERVED: uint64_t = !(FLAGS_NON_SECURITY_BITS
                                                   | SGX_FLAGS_INITTED
                                                   | SGX_FLAGS_DEBUG
                                                   | SGX_FLAGS_KSS);
pub const MISC_NON_SECURITY_BITS: uint32_t     = 0x0FFF_FFFF;
pub const TSEAL_DEFAULT_MISCMASK: uint32_t     = !MISC_NON_SECURITY_BITS;


//
// sgx_dh.h
//
pub const SGX_DH_MAC_SIZE: size_t           = 16;
pub const SGX_DH_SESSION_DATA_SIZE: size_t  = 200;

impl_packed_copy_clone! {
    pub struct sgx_dh_msg1_t {
        pub g_a: sgx_ec256_public_t,
        pub target: sgx_target_info_t,
    }

    pub struct sgx_dh_msg2_t {
        pub g_b: sgx_ec256_public_t,
        pub report: sgx_report_t,
        pub cmac: [uint8_t; SGX_DH_MAC_SIZE],
    }

    pub struct sgx_dh_msg3_body_t {
        pub report: sgx_report_t,
        pub additional_prop_length: uint32_t,
        pub additional_prop: [uint8_t; 0],
    }

    pub struct sgx_dh_msg3_t {
        pub cmac: [uint8_t; SGX_DH_MAC_SIZE],
        pub msg3_body: sgx_dh_msg3_body_t,
    }

    pub struct sgx_dh_session_enclave_identity_t {
        pub cpu_svn: sgx_cpu_svn_t,
        pub misc_select: sgx_misc_select_t,
        pub reserved_1: [uint8_t; 28],
        pub attributes: sgx_attributes_t,
        pub mr_enclave: sgx_measurement_t,
        pub reserved_2: [uint8_t; 32],
        pub mr_signer: sgx_measurement_t,
        pub reserved_3: [uint8_t; 96],
        pub isv_prod_id: sgx_prod_id_t,
        pub isv_svn: sgx_isv_svn_t,
    }

    pub struct sgx_dh_session_t {
        pub sgx_dh_session: [uint8_t; SGX_DH_SESSION_DATA_SIZE],
    }
}

impl_struct_default! {
    sgx_dh_msg1_t; //576
    sgx_dh_msg2_t; //512
    sgx_dh_msg3_body_t; //436
    sgx_dh_msg3_t; //452
    sgx_dh_session_enclave_identity_t; //260
    sgx_dh_session_t; //200
}

impl_struct_ContiguousMemory! {
    sgx_dh_msg1_t;
    sgx_dh_msg2_t;
    sgx_dh_msg3_body_t;
    sgx_dh_msg3_t;
    sgx_dh_session_enclave_identity_t;
    sgx_dh_session_t;
}

impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_dh_session_role_t {
        SGX_DH_SESSION_INITIATOR = 0,
        SGX_DH_SESSION_RESPONDER = 1,
    }
}

//
// sgx_ecp_types.h
//
pub const SGX_FEBITSIZE: uint32_t = 256;

impl_packed_struct! {
    pub struct sgx_ecc_param_t {
        pub eccP: [uint32_t; SGX_NISTP_ECP256_KEY_SIZE],      /* EC prime field */
        pub eccA: [uint32_t; SGX_NISTP_ECP256_KEY_SIZE],      /* EC curve coefficient A */
        pub eccB: [uint32_t; SGX_NISTP_ECP256_KEY_SIZE],      /* EC curve coefficient B */
        pub eccG: [[uint32_t; SGX_NISTP_ECP256_KEY_SIZE]; 2], /* ECC base point */
        pub eccR: [uint32_t; SGX_NISTP_ECP256_KEY_SIZE],      /* ECC base point order */
    }
}

pub type sgx_ec_key_128bit_t = [uint8_t; SGX_CMAC_KEY_SIZE];

//
// sgx_eid.h
//
pub type sgx_enclave_id_t = uint64_t;

//
// sgx_key.h
//
// Key Name
pub const SGX_KEYSELECT_LICENSE: uint16_t          = 0x0000;
pub const SGX_KEYSELECT_PROVISION: uint16_t        = 0x0001;
pub const SGX_KEYSELECT_PROVISION_SEAL: uint16_t   = 0x0002;
pub const SGX_KEYSELECT_REPORT: uint16_t           = 0x0003;
pub const SGX_KEYSELECT_SEAL: uint16_t             = 0x0004;

// Key Policy
pub const SGX_KEYPOLICY_MRENCLAVE: uint16_t        = 0x0001;      /* Derive key using the enclave's ENCLAVE measurement register */
pub const SGX_KEYPOLICY_MRSIGNER: uint16_t         = 0x0002;      /* Derive key using the enclave's SINGER measurement register */
pub const SGX_KEYPOLICY_NOISVPRODID: uint16_t      = 0x0004;      /* Derive key without the enclave's ISVPRODID */
pub const SGX_KEYPOLICY_CONFIGID: uint16_t         = 0x0008;      /* Derive key with the enclave's CONFIGID */
pub const SGX_KEYPOLICY_ISVFAMILYID: uint16_t      = 0x0010;      /* Derive key with the enclave's ISVFAMILYID */
pub const SGX_KEYPOLICY_ISVEXTPRODID: uint16_t     = 0x0020;      /* Derive key with the enclave's ISVEXTPRODID */

pub const SGX_KEYID_SIZE: size_t                    = 32;
pub const SGX_CPUSVN_SIZE: size_t                   = 16;
pub const SGX_CONFIGID_SIZE: size_t                 = 64;
pub const SGX_KEY_REQUEST_RESERVED2_BYTES: size_t   = 434;

pub type sgx_key_128bit_t = [uint8_t; 16];
pub type sgx_isv_svn_t = uint16_t;
pub type sgx_config_svn_t = uint16_t;
pub type sgx_config_id_t = [uint8_t; SGX_CONFIGID_SIZE];

impl_struct! {
    pub struct sgx_cpu_svn_t {
        pub svn: [uint8_t; SGX_CPUSVN_SIZE],
    }

    pub struct sgx_key_id_t {
        pub id: [uint8_t; SGX_KEYID_SIZE],
    }
}

impl_copy_clone! {
    pub struct sgx_key_request_t {
        pub key_name: uint16_t,
        pub key_policy: uint16_t,
        pub isv_svn: sgx_isv_svn_t,
        pub reserved1: uint16_t,
        pub cpu_svn: sgx_cpu_svn_t,
        pub attribute_mask: sgx_attributes_t,
        pub key_id: sgx_key_id_t,
        pub misc_mask: sgx_misc_select_t,
        pub config_svn: sgx_config_svn_t,
        pub reserved2: [uint8_t; SGX_KEY_REQUEST_RESERVED2_BYTES],
    }
}

impl_struct_default! {
    sgx_key_request_t; //512
}

impl_struct_ContiguousMemory! {
    sgx_key_request_t;
}

//
// sgx_key_exchange.h
//
pub type sgx_ra_context_t = uint32_t;
pub type sgx_ra_key_128_t = sgx_key_128bit_t;

impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_ra_key_type_t {
        SGX_RA_KEY_SK = 1,
        SGX_RA_KEY_MK = 2,
    }
}

impl_struct! {
    pub struct sgx_ra_msg1_t {
        pub g_a: sgx_ec256_public_t,
        pub gid: sgx_epid_group_id_t,
    }

    pub struct sgx_ra_msg2_t {
        pub g_b: sgx_ec256_public_t,
        pub spid: sgx_spid_t,
        pub quote_type: uint16_t,
        pub kdf_id: uint16_t,
        pub sign_gb_ga: sgx_ec256_signature_t,
        pub mac: sgx_mac_t,
        pub sig_rl_size: uint32_t,
        pub sig_rl: [uint8_t; 0],
    }
}

impl_copy_clone! {
    /* intel sgx sdk 2.8 */
    pub struct sgx_ps_sec_prop_desc_t {
        pub sgx_ps_sec_prop_desc: [uint8_t; 256],
    }

    pub struct sgx_ra_msg3_t {
        pub mac: sgx_mac_t,
        pub g_a: sgx_ec256_public_t,
        pub ps_sec_prop: sgx_ps_sec_prop_desc_t,
        pub quote: [uint8_t; 0],
    }
}

impl_struct_default! {
    sgx_ps_sec_prop_desc_t; //256
    sgx_ra_msg3_t; //336
}

impl_struct_ContiguousMemory! {
    sgx_ps_sec_prop_desc_t;
    sgx_ra_msg3_t;
}

//
// sgx_quote.h
//
pub type sgx_epid_group_id_t = [uint8_t; 4];
pub const SGX_PLATFORM_INFO_SIZE: size_t = 101;

impl_packed_struct! {
    pub struct sgx_spid_t {
        pub id: [uint8_t; 16],
    }

    pub struct sgx_basename_t {
        pub name: [uint8_t; 32],
    }

    pub struct sgx_quote_nonce_t {
        pub rand: [uint8_t; 16],
    }

    pub struct sgx_update_info_bit_t {
        pub ucodeUpdate: int32_t,
        pub csmeFwUpdate: int32_t,
        pub pswUpdate: int32_t,
    }
}

impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_quote_sign_type_t {
        SGX_UNLINKABLE_SIGNATURE    = 0,
        SGX_LINKABLE_SIGNATURE      = 1,
    }
}

impl_packed_copy_clone! {
    pub struct sgx_quote_t {
        pub version: uint16_t,                    /* 0   */
        pub sign_type: uint16_t,                  /* 2   */
        pub epid_group_id: sgx_epid_group_id_t,   /* 4   */
        pub qe_svn: sgx_isv_svn_t,                /* 8   */
        pub pce_svn: sgx_isv_svn_t,               /* 10  */
        pub xeid: uint32_t,                       /* 12  */
        pub basename: sgx_basename_t,             /* 16  */
        pub report_body: sgx_report_body_t,       /* 48  */
        pub signature_len: uint32_t,              /* 432 */
        pub signature: [uint8_t; 0],              /* 436 */
    }

    pub struct sgx_platform_info_t {
        pub platform_info: [uint8_t; SGX_PLATFORM_INFO_SIZE],
    }

    /* intel sgx sdk 2.5 */
    pub struct sgx_att_key_id_t {
        pub att_key_id: [uint8_t; 256],
    }

    /* intel sgx sdk 2.9.1 */
    /* sgx_ql_att_key_id_t moved from sgx_quote_3.h to sgx_quote.h */
    /* Describes a single attestation key. Contains both QE identity and the attestation algorithm ID. */
    pub struct sgx_ql_att_key_id_t {
        pub id: uint16_t,                       //< Structure ID
        pub version: uint16_t,                  //< Structure version
        pub mrsigner_length: uint16_t,          //< Number of valid bytes in MRSIGNER.
        pub mrsigner: [uint8_t; 48],            //< SHA256 or SHA384 hash of the Public key that signed the QE.
                                                //< The lower bytes contain MRSIGNER.  Bytes beyond mrsigner_length '0'
        pub prod_id: uint32_t,                  //< Legacy Product ID of the QE
        pub extended_prod_id: [uint8_t; 16],    //< Extended Product ID or the QE. All 0's for legacy format enclaves.
        pub config_id: [uint8_t; 64],           //< Config ID of the QE.
        pub family_id: [uint8_t; 16],           //< Family ID of the QE.
        pub algorithm_id: uint32_t,             //< Identity of the attestation key algorithm.
    }

    /* intel sgx sdk 2.9.1 */
    /* sgx_att_key_id_ext_t moved from sgx_quote_3.h to sgx_quote.h */
    /* Describes an extended attestation key. Contains sgx_ql_att_key_id_t, spid and quote_type */
    pub struct sgx_att_key_id_ext_t {
        pub base: sgx_ql_att_key_id_t,
        pub spid: [uint8_t; 16],                //< Service Provider ID, should be 0s for ECDSA quote
        pub att_key_type: uint16_t,             //< For non-EPID quote, it should be 0
                                                //< For EPID quote, it equals to sgx_quote_sign_type_t
        pub reserved: [uint8_t; 80],            //< It should have the same size of sgx_att_key_id_t
    }

    pub struct sgx_qe_report_info_t {
        pub nonce: sgx_quote_nonce_t,
        pub app_enclave_target_info: sgx_target_info_t,
        pub qe_report: sgx_report_t,
    }
}

impl_struct_default! {
    sgx_quote_t; //436
    sgx_platform_info_t; //101
    sgx_att_key_id_t; //256
    sgx_ql_att_key_id_t; //158
    sgx_att_key_id_ext_t; //256
    sgx_qe_report_info_t; //960
}

impl_struct_ContiguousMemory! {
    sgx_quote_t;
    sgx_platform_info_t;
    sgx_att_key_id_t;
    sgx_qe_report_info_t;
    sgx_ql_att_key_id_t;
    sgx_att_key_id_ext_t;
}

//
// sgx_report.h
//
pub const SGX_HASH_SIZE: size_t = 32;
pub const SGX_MAC_SIZE: size_t = 16;

pub const SGX_REPORT_DATA_SIZE: size_t = 64;

pub const SGX_ISVEXT_PROD_ID_SIZE: size_t = 16;
pub const SGX_ISV_FAMILY_ID_SIZE: size_t  = 16;

pub type sgx_isvext_prod_id_t = [uint8_t; SGX_ISVEXT_PROD_ID_SIZE];
pub type sgx_isvfamily_id_t = [uint8_t; SGX_ISV_FAMILY_ID_SIZE];

impl_struct! {
    pub struct sgx_measurement_t {
        pub m: [uint8_t; SGX_HASH_SIZE],
    }
}

pub type sgx_mac_t = [uint8_t; SGX_MAC_SIZE];

impl_copy_clone! {
    pub struct sgx_report_data_t {
        pub d: [uint8_t; SGX_REPORT_DATA_SIZE],
    }
}

impl_struct_default! {
    sgx_report_data_t; //64
}

impl_struct_ContiguousMemory! {
    sgx_report_data_t;
}

pub type sgx_prod_id_t = uint16_t;

pub const SGX_TARGET_INFO_RESERVED1_BYTES: size_t = 2;
pub const SGX_TARGET_INFO_RESERVED2_BYTES: size_t = 8;
pub const SGX_TARGET_INFO_RESERVED3_BYTES: size_t = 384;

pub const SGX_REPORT_BODY_RESERVED1_BYTES: size_t = 12;
pub const SGX_REPORT_BODY_RESERVED2_BYTES: size_t = 32;
pub const SGX_REPORT_BODY_RESERVED3_BYTES: size_t = 32;
pub const SGX_REPORT_BODY_RESERVED4_BYTES: size_t = 42;

impl_copy_clone! {
    pub struct sgx_target_info_t {
        pub mr_enclave: sgx_measurement_t,
        pub attributes: sgx_attributes_t,
        pub reserved1: [uint8_t; SGX_TARGET_INFO_RESERVED1_BYTES],
        pub config_svn: sgx_config_svn_t,
        pub misc_select: sgx_misc_select_t,
        pub reserved2: [uint8_t; SGX_TARGET_INFO_RESERVED2_BYTES],
        pub config_id: sgx_config_id_t,
        pub reserved3: [uint8_t; SGX_TARGET_INFO_RESERVED3_BYTES],
    }

    pub struct sgx_report_body_t {
        pub cpu_svn: sgx_cpu_svn_t,
        pub misc_select: sgx_misc_select_t,
        pub reserved1: [uint8_t; SGX_REPORT_BODY_RESERVED1_BYTES],
        pub isv_ext_prod_id: sgx_isvext_prod_id_t,
        pub attributes: sgx_attributes_t,
        pub mr_enclave: sgx_measurement_t,
        pub reserved2: [uint8_t; SGX_REPORT_BODY_RESERVED2_BYTES],
        pub mr_signer: sgx_measurement_t,
        pub reserved3: [uint8_t; SGX_REPORT_BODY_RESERVED3_BYTES],
        pub config_id: sgx_config_id_t,
        pub isv_prod_id: sgx_prod_id_t,
        pub isv_svn: sgx_isv_svn_t,
        pub config_svn: sgx_config_svn_t,
        pub reserved4: [uint8_t; SGX_REPORT_BODY_RESERVED4_BYTES],
        pub isv_family_id: sgx_isvfamily_id_t,
        pub report_data: sgx_report_data_t,
    }

    pub struct sgx_report_t {
        pub body: sgx_report_body_t,
        pub key_id: sgx_key_id_t,
        pub mac: sgx_mac_t,
    }
}

impl_struct_default! {
    sgx_target_info_t; //512
    sgx_report_body_t; //384
    sgx_report_t; //432
}

impl_struct_ContiguousMemory! {
    sgx_target_info_t;
    sgx_report_body_t;
    sgx_report_t;
}

//
// sgx_spinlock.h
//
pub type sgx_spinlock_t = uint32_t;

pub const SGX_SPINLOCK_INITIALIZER: uint32_t = 0;

//
// sgx_tae_service.h
//
/* delete intel sgx sdk2.8 */
/*
pub type sgx_time_t = uint64_t;

pub type sgx_time_source_nonce_t = [uint8_t; 32];

pub const SGX_MC_UUID_COUNTER_ID_SIZE: size_t    = 3;
pub const SGX_MC_UUID_NONCE_SIZE: size_t         = 13;

impl_struct! {
    #[repr(packed)]
    pub struct sgx_mc_uuid_t {
        pub counter_id: [uint8_t; SGX_MC_UUID_COUNTER_ID_SIZE],
        pub nonce: [uint8_t; SGX_MC_UUID_NONCE_SIZE],
    }
}

impl_copy_clone! {
    #[repr(packed)]
    pub struct sgx_ps_sec_prop_desc_t {
        pub sgx_ps_sec_prop_desc: [uint8_t; 256],
    }

    pub struct sgx_ps_sec_prop_desc_ex_t {
        pub ps_sec_prop_desc: sgx_ps_sec_prop_desc_t,
        pub pse_mrsigner: sgx_measurement_t,
        pub pse_prod_id: sgx_prod_id_t,
        pub pse_isv_svn: sgx_isv_svn_t,
    }
}

impl_struct_default! {
    sgx_ps_sec_prop_desc_t; //256
    sgx_ps_sec_prop_desc_ex_t; //292
}

impl_struct_ContiguousMemory! {
    sgx_ps_sec_prop_desc_t;
    sgx_ps_sec_prop_desc_ex_t;
}

pub const SGX_MC_POLICY_SIGNER: uint16_t   = 0x01;
pub const SGX_MC_POLICY_ENCLAVE: uint16_t  = 0x02;
*/

//
// sgx_tcrypto.h
//
pub const SGX_SHA1_HASH_SIZE: size_t         = 20;
pub const SGX_SHA256_HASH_SIZE: size_t       = 32;
pub const SGX_ECP256_KEY_SIZE: size_t        = 32;
pub const SGX_NISTP_ECP256_KEY_SIZE: size_t  = SGX_ECP256_KEY_SIZE / 4;
pub const SGX_AESGCM_IV_SIZE: size_t         = 12;
pub const SGX_AESGCM_KEY_SIZE: size_t        = 16;
pub const SGX_AESGCM_MAC_SIZE: size_t        = 16;
pub const SGX_HMAC256_KEY_SIZE: size_t       = 32;
pub const SGX_HMAC256_MAC_SIZE: size_t       = 32;
pub const SGX_CMAC_KEY_SIZE: size_t          = 16;
pub const SGX_CMAC_MAC_SIZE: size_t          = 16;
pub const SGX_AESCTR_KEY_SIZE: size_t        = 16;
pub const SGX_RSA3072_KEY_SIZE: size_t       = 384;
pub const SGX_RSA3072_PRI_EXP_SIZE: size_t   = 384;
pub const SGX_RSA3072_PUB_EXP_SIZE: size_t   = 4;

impl_struct! {
    pub struct sgx_ec256_dh_shared_t {
        pub s: [uint8_t; SGX_ECP256_KEY_SIZE],
    }

    /* delete (intel sgx sdk 2.0)
    pub struct sgx_ec256_dh_shared512_t {
        pub x: [uint8_t; SGX_ECP256_KEY_SIZE],
        pub y: [uint8_t; SGX_ECP256_KEY_SIZE],
    }
    */

    pub struct sgx_ec256_private_t {
        pub r: [uint8_t; SGX_ECP256_KEY_SIZE],
    }

    pub struct sgx_ec256_public_t {
        pub gx: [uint8_t; SGX_ECP256_KEY_SIZE],
        pub gy: [uint8_t; SGX_ECP256_KEY_SIZE],
    }

    pub struct sgx_ec256_signature_t {
        pub x: [uint32_t; SGX_NISTP_ECP256_KEY_SIZE],
        pub y: [uint32_t; SGX_NISTP_ECP256_KEY_SIZE],
    }
}

impl_copy_clone! {
    pub struct sgx_rsa3072_public_key_t {
        pub modulus: [uint8_t; SGX_RSA3072_KEY_SIZE],
        pub exponent: [uint8_t; SGX_RSA3072_PUB_EXP_SIZE],
    }

    /* intel sgx sdk 2.0 */
    pub struct sgx_rsa3072_key_t {
        pub modulus: [uint8_t; SGX_RSA3072_KEY_SIZE],
        pub d: [uint8_t; SGX_RSA3072_PRI_EXP_SIZE],
        pub e: [uint8_t; SGX_RSA3072_PUB_EXP_SIZE],
    }

    /* intel sgx sdk 1.9 */
    /*
    pub struct sgx_rsa3072_private_key_t {
        pub modulus: [uint8_t; SGX_RSA3072_KEY_SIZE],
        pub exponent: [uint8_t; SGX_RSA3072_PRI_EXP_SIZE],
    }
    */

    pub struct sgx_rsa3072_signature_t {
        pub signature: [uint8_t; SGX_RSA3072_KEY_SIZE],
    }
}

impl_struct_default! {
    sgx_rsa3072_public_key_t; //388
    sgx_rsa3072_key_t; //772
    sgx_rsa3072_signature_t; //384
}

impl_struct_ContiguousMemory! {
    sgx_rsa3072_public_key_t;
    sgx_rsa3072_key_t;
    sgx_rsa3072_signature_t;
}

//pub type sgx_rsa3072_signature_t    = [uint8_t; SGX_RSA3072_KEY_SIZE];

pub type sgx_sha_state_handle_t     = *mut c_void;
pub type sgx_hmac_state_handle_t    = *mut c_void;
pub type sgx_cmac_state_handle_t    = *mut c_void;
pub type sgx_ecc_state_handle_t     = *mut c_void;
pub type sgx_aes_state_handle_t     = *mut c_void;

pub type sgx_sha1_hash_t = [uint8_t; SGX_SHA1_HASH_SIZE];
pub type sgx_sha256_hash_t = [uint8_t; SGX_SHA256_HASH_SIZE];

pub type sgx_aes_gcm_128bit_key_t = [uint8_t; SGX_AESGCM_KEY_SIZE];
pub type sgx_aes_gcm_128bit_tag_t = [uint8_t; SGX_AESGCM_MAC_SIZE];
pub type sgx_hmac_256bit_key_t = [uint8_t; SGX_HMAC256_KEY_SIZE];
pub type sgx_hmac_256bit_tag_t = [uint8_t; SGX_HMAC256_MAC_SIZE];
pub type sgx_cmac_128bit_key_t = [uint8_t; SGX_CMAC_KEY_SIZE];
pub type sgx_cmac_128bit_tag_t = [uint8_t; SGX_CMAC_MAC_SIZE];
pub type sgx_aes_ctr_128bit_key_t = [uint8_t; SGX_AESCTR_KEY_SIZE];

impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_generic_ecresult_t {
        SGX_EC_VALID                = 0x0000_0000,   /* validation pass successfully */
        SGX_EC_COMPOSITE_BASE       = 0x0000_0001,   /* field based on composite */
        SGX_EC_COMPLICATED_BASE     = 0x0000_0002,   /* number of non-zero terms in the polynomial (> PRIME_ARR_MAX) */
        SGX_EC_IS_ZERO_DISCRIMINANT = 0x0000_0003,   /* zero discriminant */
        SGX_EC_COMPOSITE_ORDER      = 0x0000_0004,   /* composite order of base point */
        SGX_EC_INVALID_ORDER        = 0x0000_0005,   /* invalid base point order */
        SGX_EC_IS_WEAK_MOV          = 0x0000_0006,   /* weak Meneze-Okamoto-Vanstone  reduction attack */
        SGX_EC_IS_WEAK_SSA          = 0x0000_0007,   /* weak Semaev-Smart,Satoh-Araki reduction attack */
        SGX_EC_IS_SUPER_SINGULAR    = 0x0000_0008,   /* supersingular curve */
        SGX_EC_INVALID_PRIVATE_KEY  = 0x0000_0009,   /* !(0 < Private < order) */
        SGX_EC_INVALID_PUBLIC_KEY   = 0x0000_000A,   /* (order*PublicKey != Infinity) */
        SGX_EC_INVALID_KEY_PAIR     = 0x0000_000B,   /* (Private*BasePoint != PublicKey) */
        SGX_EC_POINT_OUT_OF_GROUP   = 0x0000_000C,   /* out of group (order*P != Infinity) */
        SGX_EC_POINT_IS_AT_INFINITY = 0x0000_000D,   /* point (P=(Px,Py)) at Infinity */
        SGX_EC_POINT_IS_NOT_VALID   = 0x0000_000E,   /* point (P=(Px,Py)) out-of EC */
        SGX_EC_POINT_IS_EQUAL       = 0x0000_000F,   /* compared points are equal */
        SGX_EC_POINT_IS_NOT_EQUAL   = 0x0000_0010,   /* compared points are different */
        SGX_EC_INVALID_SIGNATURE    = 0x0000_0011,   /* invalid signature */
    }
}

impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_rsa_result_t {
        SGX_RSA_VALID               = 0,   /* validation pass successfully */
        SGX_RSA_INVALID_SIGNATURE   = 1,   /* invalid signature */
    }
}

/* intel sgx sdk 2.1.3 */
impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_rsa_key_type_t {
        SGX_RSA_PRIVATE_KEY     = 0,   /* RSA private key state */
        SGX_RSA_PUBLIC_KEY      = 1,   /* RSA public key state */
    }
}

/* intel sgx sdk 2.1.3 */
pub const N_SIZE_IN_BYTES: size_t = 384;
pub const E_SIZE_IN_BYTES: size_t = 4;
pub const D_SIZE_IN_BYTES: size_t = 384;
pub const P_SIZE_IN_BYTES: size_t = 192;
pub const Q_SIZE_IN_BYTES: size_t = 192;
pub const DMP1_SIZE_IN_BYTES: size_t = 192;
pub const DMQ1_SIZE_IN_BYTES: size_t = 192;
pub const IQMP_SIZE_IN_BYTES: size_t = 192;

/* intel sgx sdk 2.1.3 */
pub const N_SIZE_IN_UINT: size_t = N_SIZE_IN_BYTES / 4;
pub const E_SIZE_IN_UINT: size_t = E_SIZE_IN_BYTES / 4;
pub const D_SIZE_IN_UINT: size_t = D_SIZE_IN_BYTES / 4;
pub const P_SIZE_IN_UINT: size_t = P_SIZE_IN_BYTES / 4;
pub const Q_SIZE_IN_UINT: size_t = Q_SIZE_IN_BYTES / 4;
pub const DMP1_SIZE_IN_UINT: size_t = DMP1_SIZE_IN_BYTES / 4;
pub const DMQ1_SIZE_IN_UINT: size_t = DMQ1_SIZE_IN_BYTES / 4;
pub const IQMP_SIZE_IN_UINT: size_t = IQMP_SIZE_IN_BYTES / 4;

pub type sgx_rsa_key_t = *mut c_void;

/* intel sgx sdk 2.1.3 */
impl_copy_clone! {
    pub struct rsa_params_t {
        pub n: [uint32_t; N_SIZE_IN_UINT],
        pub e: [uint32_t; E_SIZE_IN_UINT],
        pub d: [uint32_t; D_SIZE_IN_UINT],
        pub p: [uint32_t; P_SIZE_IN_UINT],
        pub q: [uint32_t; Q_SIZE_IN_UINT],
        pub dmp1: [uint32_t; DMP1_SIZE_IN_UINT],
        pub dmq1: [uint32_t; DMQ1_SIZE_IN_UINT],
        pub iqmp: [uint32_t; IQMP_SIZE_IN_UINT],
    }
}

impl_struct_default! {
    rsa_params_t; //1732
}

impl_struct_ContiguousMemory! {
    rsa_params_t;
}

//
// sgx_thread.h
//
pub type sgx_thread_t = uintptr_t;

cfg_if! {
    if #[cfg(target_arch = "x86")] {
        pub const SE_WORDSIZE: size_t = 4;
    } else {
        pub const SE_WORDSIZE: size_t = 8;
    }
}

#[repr(C)]
pub struct sgx_thread_queue_t {
    pub m_first: sgx_thread_t,
    pub m_last: sgx_thread_t,
}

#[repr(C)]
pub struct sgx_thread_mutex_t {
    pub m_refcount: size_t,
    pub m_control: uint32_t,
    pub m_lock: uint32_t,
    pub m_owner: sgx_thread_t,
    pub m_queue: sgx_thread_queue_t,
}

#[repr(C)]
pub struct sgx_thread_rwlock_t {
    pub m_reader_count: uint32_t,
    pub m_writers_waiting: uint32_t,
    pub m_lock: uint32_t,
    pub m_owner: sgx_thread_t,
    pub m_reader_queue: sgx_thread_queue_t,
    pub m_writer_queue: sgx_thread_queue_t,
}

pub const SGX_THREAD_T_NULL: sgx_thread_t = 0;

pub const SGX_THREAD_MUTEX_NONRECURSIVE: uint32_t = 0x01;
pub const SGX_THREAD_MUTEX_RECURSIVE: uint32_t = 0x02;

pub const SGX_THREAD_NONRECURSIVE_MUTEX_INITIALIZER: sgx_thread_mutex_t = sgx_thread_mutex_t {
    m_refcount: 0,
    m_control: SGX_THREAD_MUTEX_NONRECURSIVE,
    m_lock: 0,
    m_owner: SGX_THREAD_T_NULL,
    m_queue: sgx_thread_queue_t {
        m_first: SGX_THREAD_T_NULL,
        m_last: SGX_THREAD_T_NULL,
    },
};

pub const SGX_THREAD_RECURSIVE_MUTEX_INITIALIZER: sgx_thread_mutex_t = sgx_thread_mutex_t {
    m_refcount: 0,
    m_control: SGX_THREAD_MUTEX_RECURSIVE,
    m_lock: 0,
    m_owner: SGX_THREAD_T_NULL,
    m_queue: sgx_thread_queue_t {
        m_first: SGX_THREAD_T_NULL,
        m_last: SGX_THREAD_T_NULL,
    },
};

pub const SGX_THREAD_MUTEX_INITIALIZER: sgx_thread_mutex_t =
    SGX_THREAD_NONRECURSIVE_MUTEX_INITIALIZER;

pub const SGX_THREAD_LOCK_INITIALIZER: sgx_thread_rwlock_t = sgx_thread_rwlock_t {
    m_reader_count: 0,
    m_writers_waiting: 0,
    m_lock: 0,
    m_owner: SGX_THREAD_T_NULL,
    m_reader_queue: sgx_thread_queue_t {
        m_first: SGX_THREAD_T_NULL,
        m_last: SGX_THREAD_T_NULL,
    },
    m_writer_queue: sgx_thread_queue_t {
        m_first: SGX_THREAD_T_NULL,
        m_last: SGX_THREAD_T_NULL,
    },
};

impl_struct! {
    pub struct sgx_thread_mutex_attr_t {
        pub m_dummy: c_uchar,
    }

    pub struct sgx_thread_rwlockattr_t {
        pub m_dummy: c_uchar,
    }

    pub struct sgx_thread_cond_attr_t {
        pub m_dummy: c_uchar,
    }
}

#[repr(C)]
pub struct sgx_thread_cond_t {
    pub m_lock: uint32_t,
    pub m_queue: sgx_thread_queue_t,
}

pub const SGX_THREAD_COND_INITIALIZER: sgx_thread_cond_t = sgx_thread_cond_t {
    m_lock: 0,
    m_queue: sgx_thread_queue_t {
        m_first: SGX_THREAD_T_NULL,
        m_last: SGX_THREAD_T_NULL,
    },
};

//
// sgx_tkey_exchange.h
//
pub type sgx_ra_derive_secret_keys_t = extern "C" fn(
    p_shared_key: *const sgx_ec256_dh_shared_t,
    kdf_id: uint16_t,
    p_smk_key: *mut sgx_ec_key_128bit_t,
    p_sk_key: *mut sgx_ec_key_128bit_t,
    p_mk_key: *mut sgx_ec_key_128bit_t,
    p_vk_key: *mut sgx_ec_key_128bit_t,
) -> sgx_status_t;

//
// sgx_trts_exception.h
//
pub const EXCEPTION_CONTINUE_SEARCH: int32_t = 0;
pub const EXCEPTION_CONTINUE_EXECUTION: int32_t = -1;

impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_exception_vector_t {
        SGX_EXCEPTION_VECTOR_DE = 0,  /* DIV and DIV instructions */
        SGX_EXCEPTION_VECTOR_DB = 1,  /* For Intel use only */
        SGX_EXCEPTION_VECTOR_BP = 3,  /* INT 3 instruction */
        SGX_EXCEPTION_VECTOR_BR = 5,  /* BOUND instruction */
        SGX_EXCEPTION_VECTOR_UD = 6,  /* UD2 instruction or reserved opcode */
        SGX_EXCEPTION_VECTOR_GP = 13, /* General protection exception */
        SGX_EXCEPTION_VECTOR_PF = 14, /* Page fault exception */
        SGX_EXCEPTION_VECTOR_MF = 16, /* x87 FPU floating-point or WAIT/FWAIT instruction */
        SGX_EXCEPTION_VECTOR_AC = 17, /* Any data reference in memory */
        SGX_EXCEPTION_VECTOR_XM = 19, /* SSE/SSE2/SSE3 floating-point instruction */
        SGX_EXCEPTION_VECTOR_CP = 21, /* Control protection exception */
    }
}

impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_exception_type_t {
        SGX_EXCEPTION_HARDWARE = 3,
        SGX_EXCEPTION_SOFTWARE = 6,
    }
}

cfg_if! {
    if #[cfg(target_arch = "x86")] {
        impl_struct! {
            pub struct sgx_cpu_context_t {
                pub eax: uint32_t,
                pub ecx: uint32_t,
                pub edx: uint32_t,
                pub ebx: uint32_t,
                pub esp: uint32_t,
                pub ebp: uint32_t,
                pub esi: uint32_t,
                pub edi: uint32_t,
                pub eflags: uint32_t,
                pub eip: uint32_t,
            }
        }
    } else {
        impl_struct! {
            pub struct sgx_cpu_context_t {
                pub rax: uint64_t,
                pub rcx: uint64_t,
                pub rdx: uint64_t,
                pub rbx: uint64_t,
                pub rsp: uint64_t,
                pub rbp: uint64_t,
                pub rsi: uint64_t,
                pub rdi: uint64_t,
                pub r8: uint64_t,
                pub r9: uint64_t,
                pub r10: uint64_t,
                pub r11: uint64_t,
                pub r12: uint64_t,
                pub r13: uint64_t,
                pub r14: uint64_t,
                pub r15: uint64_t,
                pub rflags: uint64_t,
                pub rip: uint64_t,
            }
        }
    }
}

impl_struct! {
    pub struct sgx_exception_info_t {
        pub cpu_context: sgx_cpu_context_t,
        pub exception_vector: sgx_exception_vector_t,
        pub exception_type: sgx_exception_type_t,
    }
}

pub type sgx_exception_handler_t = extern "C" fn(info: *mut sgx_exception_info_t) -> int32_t;

//
// sgx_tseal.h
//
pub const SGX_SEAL_TAG_SIZE: size_t = SGX_AESGCM_MAC_SIZE;
pub const SGX_SEAL_IV_SIZE: size_t = 12;

impl_struct! {
    pub struct sgx_aes_gcm_data_t {
        pub payload_size: uint32_t,
        pub reserved: [uint8_t; 12],
        pub payload_tag: [uint8_t; SGX_SEAL_TAG_SIZE],
        pub payload: [uint8_t; 0],
    }

    pub struct sgx_sealed_data_t {
        pub key_request: sgx_key_request_t,
        pub plain_text_offset: uint32_t,
        pub reserved: [uint8_t; 12],
        pub aes_data: sgx_aes_gcm_data_t,
    }
}

//
// sgx_uae_platform.h
//
pub const PS_CAP_TRUSTED_TIME: size_t = 0x1;
pub const PS_CAP_MONOTONIC_COUNTER: size_t = 0x2;

impl_struct! {
    pub struct sgx_ps_cap_t {
        pub ps_cap0: uint32_t,
        pub ps_cap1: uint32_t,
    }
}

//
// sgx_ukey_exchange.h
//
pub type sgx_ecall_get_ga_trusted_t = unsafe extern "C" fn(
    eid: sgx_enclave_id_t,
    retval: *mut sgx_status_t,
    context: sgx_ra_context_t,
    g_a: *mut sgx_ec256_public_t,
) -> sgx_status_t;

pub type sgx_ecall_proc_msg2_trusted_t = unsafe extern "C" fn(
    eid: sgx_enclave_id_t,
    retval: *mut sgx_status_t,
    context: sgx_ra_context_t,
    p_msg2: *const sgx_ra_msg2_t,
    p_qe_target: *const sgx_target_info_t,
    p_report: *mut sgx_report_t,
    nonce: *mut sgx_quote_nonce_t,
) -> sgx_status_t;

pub type sgx_ecall_get_msg3_trusted_t = unsafe extern "C" fn(
    eid: sgx_enclave_id_t,
    retval: *mut sgx_status_t,
    context: sgx_ra_context_t,
    quote_size: uint32_t,
    qe_report: *mut sgx_report_t,
    p_msg3: *mut sgx_ra_msg3_t,
    msg3_size: uint32_t,
) -> sgx_status_t;

//
// sgx_urts.h
//
pub type sgx_launch_token_t = [uint8_t; 1024];

/* intel sgx sdk 2.2 */
pub const MAX_EX_FEATURES_COUNT: size_t = 32;
pub const SGX_CREATE_ENCLAVE_EX_PCL_BIT_IDX: size_t = 0;
pub const SGX_CREATE_ENCLAVE_EX_PCL: uint32_t = 1 << SGX_CREATE_ENCLAVE_EX_PCL_BIT_IDX as uint32_t;
pub const SGX_CREATE_ENCLAVE_EX_SWITCHLESS_BIT_IDX: size_t = 1;
pub const SGX_CREATE_ENCLAVE_EX_SWITCHLESS: uint32_t =
    1 << SGX_CREATE_ENCLAVE_EX_SWITCHLESS_BIT_IDX as uint32_t;
/* intel sgx sdk 2.4 */
pub const SGX_CREATE_ENCLAVE_EX_KSS_BIT_IDX: size_t = 2;
pub const SGX_CREATE_ENCLAVE_EX_KSS: uint32_t = 1 << SGX_CREATE_ENCLAVE_EX_KSS_BIT_IDX as uint32_t;

pub const _SGX_LAST_EX_FEATURE_IDX_: uint32_t = SGX_CREATE_ENCLAVE_EX_KSS_BIT_IDX as uint32_t;
pub const _SGX_EX_FEATURES_MASK_: uint32_t =
    0xFFFF_FFFF_u32 >> (MAX_EX_FEATURES_COUNT as uint32_t - 1 - _SGX_LAST_EX_FEATURE_IDX_);

/* intel sgx sdk 2.4 */
impl_packed_copy_clone! {
    pub struct sgx_kss_config_t {
        pub config_id: sgx_config_id_t,
        pub config_svn: sgx_config_svn_t,
    }
}

impl_struct_default! {
    sgx_kss_config_t; //66
}

impl_struct_ContiguousMemory! {
    sgx_kss_config_t;
}

//
// trts.pic.h
//
pub const ENCLAVE_INIT_NOT_STARTED: uint32_t = 0;
pub const ENCLAVE_INIT_IN_PROGRESS: uint32_t = 1;
pub const ENCLAVE_INIT_DONE: uint32_t = 2;
pub const ENCLAVE_CRASHED: uint32_t = 3;

//
// sgx_cpuid.h
//
pub type sgx_cpuinfo_t = [int32_t; 4];

//
//
//
//
//cfg_if! {
//    if #[cfg(any(not(feature = "NDEBUG"), feature = "EDEBUG"))] {
//        pub const SGX_DEBUG_FLAG: int32_t   = 1;
//    } else {
//        pub const SGX_DEBUG_FLAG: int32_t   = 0;
//    }
//}

//
// sgx_tprotected_fs.h
//
pub type SGX_FILE = *mut c_void;
pub const FILENAME_MAX: c_uint = 260; //define in sgx_tprotected_fs.h
pub const FOPEN_MAX: c_uint = 20; //define in sgx_tprotected_fs.h

/* intel sgx sdk 2.0 */
//
// sgx_capable.h
//
impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_device_status_t {
        SGX_ENABLED                     = 0,
        SGX_DISABLED_REBOOT_REQUIRED    = 1,  /* A reboot is required to finish enabling SGX */
        SGX_DISABLED_LEGACY_OS          = 2,  /* SGX is disabled and a Software Control Interface is not available to enable it */
        SGX_DISABLED                    = 3,  /* SGX is not enabled on this platform. More details are unavailable */
        SGX_DISABLED_SCI_AVAILABLE      = 4,  /* SGX is disabled, but a Software Control Interface is available to enable it */
        SGX_DISABLED_MANUAL_ENABLE      = 5,  /* SGX is disabled, but can be enabled manually in the BIOS setup */
        SGX_DISABLED_HYPERV_ENABLED     = 6,  /* Detected an unsupported version of Windows* 10 with Hyper-V enabled */
        SGX_DISABLED_UNSUPPORTED_CPU    = 7,  /* SGX is not supported by this CPU */
    }
}

/* intel sgx sdk 2.1.3 */
//
// sgx_pcl_guid.h
//
pub const SGX_PCL_GUID_SIZE: size_t = 16;
pub const SGX_PCL_GUID: [uint8_t; SGX_PCL_GUID_SIZE] = [
    0x95, 0x48, 0x6e, 0x8f, 0x8f, 0x4a, 0x41, 0x4f, 0xb1, 0x27, 0x46, 0x21, 0xa8, 0x59, 0xa8, 0xac,
];

/* intel sgx sdk 2.2 */
//
// sgx_uswitchless.h
//
impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_uswitchless_worker_type_t {
        SGX_USWITCHLESS_WORKER_TYPE_UNTRUSTED  = 0,
        SGX_USWITCHLESS_WORKER_TYPE_TRUSTED    = 1,
    }
}

impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_uswitchless_worker_event_t {
        SGX_USWITCHLESS_WORKER_EVENT_START  = 0,  /* a worker thread starts */
        SGX_USWITCHLESS_WORKER_EVENT_IDLE   = 1,  /* a worker thread is idle */
        SGX_USWITCHLESS_WORKER_EVENT_MISS   = 2,  /* a worker thread misses some tasks */
        SGX_USWITCHLESS_WORKER_EVENT_EXIT   = 3,  /* a worker thread exits */
        SGX_USWITCHLESS_WORKER_EVENT_NUM    = 4,
    }
}

impl_struct! {
    pub struct sgx_uswitchless_worker_stats_t {
        pub processed: uint64_t,  /* # of tasks that all workers have processed */
        pub missed: uint64_t,     /* # of tasks that all workers have missed */
    }
}

pub type sgx_uswitchless_worker_callback_t = extern "C" fn(
    worker_type: sgx_uswitchless_worker_type_t,
    worker_event: sgx_uswitchless_worker_event_t,
    worker_stats: *const sgx_uswitchless_worker_stats_t,
);

pub const SL_DEFAULT_FALLBACK_RETRIES: uint32_t = 20000;
pub const SL_DEFAULT_SLEEP_RETRIES: uint32_t = 20000;
pub const SL_DEFUALT_MAX_TASKS_QWORDS: uint32_t = 1;
pub const SL_MAX_TASKS_MAX_QWORDS: uint32_t = 8;

pub const _SGX_USWITCHLESS_WORKER_EVENT_NUM: size_t = 4;

#[repr(C)]
pub struct sgx_uswitchless_config_t {
    pub switchless_calls_pool_size_qwords: uint32_t,
    pub num_uworkers: uint32_t,
    pub num_tworkers: uint32_t,
    pub retries_before_fallback: uint32_t,
    pub retries_before_sleep: uint32_t,
    pub callback_func: [sgx_uswitchless_worker_callback_t; _SGX_USWITCHLESS_WORKER_EVENT_NUM],
}

impl Default for sgx_uswitchless_config_t {
    fn default() -> sgx_uswitchless_config_t {
        let mut config: sgx_uswitchless_config_t = unsafe { core::mem::zeroed() };
        config.num_uworkers = 1;
        config.num_tworkers = 1;
        config
    }
}

//
// sgx_pce.h
//
/* PCE ID for the PCE in this library */
pub const PCE_ID: uint16_t = 0;
pub const PCE_ALG_RSA_OAEP_3072: uint8_t = 1;
pub const PCE_NIST_P256_ECDSA_SHA256: uint8_t = 0;

impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_ql_request_policy_t {
        SGX_QL_PERSISTENT  = 0,  /* QE is initialized on first use and reused until process ends */
        SGX_QL_EPHEMERAL   = 1,  /* QE is initialized and terminated on every quote. If a previous QE exists, it is stopped & restarted before quoting.*/
//      SGX_QL_DEFAULT     = 0,
    }
}

impl_packed_struct! {
    pub struct sgx_pce_info_t {
        pub pce_isv_svn: sgx_isv_svn_t,
        pub pce_id: uint16_t,
    }
}

//
// sgx_ql_lib_common.h
//
impl_packed_struct! {
    pub struct sgx_ql_qe3_id_t {
        pub id: [uint8_t; 16],
    }
}

impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_ql_config_version_t {
        SGX_QL_CONFIG_VERSION_1  = 1,
    }
}

#[repr(C)]
#[repr(packed)]
pub struct sgx_ql_pck_cert_id_t {
    pub p_qe3_id: *mut uint8_t,
    pub qe3_id_size: uint32_t,
    pub p_platform_cpu_svn: *mut sgx_cpu_svn_t,
    pub p_platform_pce_isv_svn: *mut sgx_isv_svn_t,
    pub p_encrypted_ppid: *mut uint8_t,
    pub encrypted_ppid_size: uint32_t,
    pub crypto_suite: uint8_t,
    pub pce_id: uint16_t,
}

#[repr(C)]
#[repr(packed)]
pub struct sgx_ql_config_t {
    pub version: sgx_ql_config_version_t,
    pub cert_cpu_svn: sgx_cpu_svn_t,
    pub cert_pce_isv_svn: sgx_isv_svn_t,
    pub cert_data_size: uint32_t,
    pub p_cert_data: *mut uint8_t,
}

#[repr(C)]
pub struct sgx_ql_qve_collateral_t {
    pub version: uint32_t, // version = 1.  PCK Cert chain is in the Quote.
    pub pck_crl_issuer_chain: *mut c_char,
    pub pck_crl_issuer_chain_size: uint32_t,
    pub root_ca_crl: *mut c_char, // Root CA CRL
    pub root_ca_crl_size: uint32_t,
    pub pck_crl: *mut c_char, // PCK Cert CRL
    pub pck_crl_size: uint32_t,
    pub tcb_info_issuer_chain: *mut c_char,
    pub tcb_info_issuer_chain_size: uint32_t,
    pub tcb_info: *mut c_char, // TCB Info structure
    pub tcb_info_size: uint32_t,
    pub qe_identity_issuer_chain: *mut c_char,
    pub qe_identity_issuer_chain_size: uint32_t,
    pub qe_identity: *mut c_char, // QE Identity Structure
    pub qe_identity_size: uint32_t,
}

/* intel DCAP 2.14 */
impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_ql_log_level_t {
        SGX_QL_LOG_ERROR  = 0,
        SGX_QL_LOG_INFO   = 1,
    }
}

pub type sgx_ql_logging_callback_t = extern "C" fn(level: sgx_ql_log_level_t, message: *const c_char);

//
// sgx_quote_3.h
//
pub const REF_QUOTE_MAX_AUTHENTICATON_DATA_SIZE: uint16_t = 64;

impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_ql_attestation_algorithm_id_t {
        SGX_QL_ALG_EPID         = 0,
        SGX_QL_ALG_RESERVED_1   = 1,
        SGX_QL_ALG_ECDSA_P256   = 2,
        SGX_QL_ALG_ECDSA_P384   = 3,
        SGX_QL_ALG_MAX          = 4,
    }
}

impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_ql_cert_key_type_t {
        PPID_CLEARTEXT          = 1,
        PPID_RSA2048_ENCRYPTED  = 2,
        PPID_RSA3072_ENCRYPTED  = 3,
        PCK_CLEARTEXT           = 4,
        PCK_CERT_CHAIN          = 5,
        ECDSA_SIG_AUX_DATA      = 6,
        QL_CERT_KEY_TYPE_MAX    = 16,
    }
}

impl_packed_copy_clone! {
    pub struct sgx_ql_ppid_rsa3072_encrypted_cert_info_t {
        pub enc_ppid: [uint8_t; 384],
        pub cpu_svn: sgx_cpu_svn_t,
        pub pce_info: sgx_pce_info_t,
    }
}

impl_struct_default! {
    sgx_ql_ppid_rsa3072_encrypted_cert_info_t; //404
}

impl_struct_ContiguousMemory! {
    sgx_ql_ppid_rsa3072_encrypted_cert_info_t;
}

impl_packed_struct! {
    pub struct sgx_ql_auth_data_t {
        pub size: uint16_t,
        pub auth_data: [uint8_t; 0],
    }

    pub struct sgx_ql_certification_data_t {
        pub cert_key_type: uint16_t,
        pub size: uint32_t,
        pub certification_data: [uint8_t; 0],
    }
}

impl_packed_copy_clone! {
    pub struct sgx_ql_ecdsa_sig_data_t {
        pub sig: [uint8_t; 64],
        pub attest_pub_key: [uint8_t; 64],
        pub qe3_report: sgx_report_body_t,
        pub qe3_report_sig: [uint8_t; 64],
        pub auth_certification_data: [uint8_t; 0],
    }
}

impl_struct_default! {
    sgx_ql_ecdsa_sig_data_t; //576
}

impl_struct_ContiguousMemory! {
    sgx_ql_ecdsa_sig_data_t;
}

impl_packed_struct! {
    pub struct sgx_quote_header_t {
        pub version: uint16_t,
        pub att_key_type: uint16_t,
        pub att_key_data_0: uint32_t,
        pub qe_svn: sgx_isv_svn_t,
        pub pce_svn: sgx_isv_svn_t,
        pub vendor_id: [uint8_t; 16],
        pub user_data: [uint8_t; 20],
    }
}

impl_packed_copy_clone! {
    pub struct sgx_quote3_t {
        pub header: sgx_quote_header_t,
        pub report_body: sgx_report_body_t,
        pub signature_data_len: uint32_t,
        pub signature_data: [uint8_t; 0],
    }
}

impl_struct_default! {
    sgx_quote3_t; //436
}

impl_struct_ContiguousMemory! {
    sgx_quote3_t;
}

//
// sgx_ql_quote.h
//
impl_packed_copy_clone! {
    pub struct sgx_ql_qe_report_info_t {
        pub nonce: sgx_quote_nonce_t,
        pub app_enclave_target_info: sgx_target_info_t,
        pub qe_report: sgx_report_t,
    }
}

impl_struct_default! {
    sgx_ql_qe_report_info_t; //960
}

impl_struct_ContiguousMemory! {
    sgx_ql_qe_report_info_t;
}

/* intel DCAP 1.6 */
//
// sgx_dcap_ql_wrapper.h
//
impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_ql_path_type_t {
        SGX_QL_QE3_PATH = 0,
        SGX_QL_PCE_PATH = 1,
        SGX_QL_QPL_PATH = 2,
    }
}

//
// qve_header.h
//

pub const ROOT_KEY_ID_SIZE: usize = 48;
pub const PLATFORM_INSTANCE_ID_SIZE: usize = 16;

/* intel DCAP 1.7 */
impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum pck_cert_flag_enum_t {
        PCK_FLAG_FALSE = 0,
        PCK_FLAG_TRUE = 1,
        PCK_FLAG_UNDEFINED = 2,
    }
}

impl_copy_clone! {
    pub struct sgx_ql_qv_supplemental_t {
        pub version: uint32_t,
        pub earliest_issue_date: time_t,
        pub latest_issue_date: time_t,
        pub earliest_expiration_date: time_t,
        pub tcb_level_date_tag: time_t,
        pub pck_crl_num: uint32_t,
        pub root_ca_crl_num: uint32_t,
        pub tcb_eval_ref_num: uint32_t,
        pub root_key_id: [uint8_t; ROOT_KEY_ID_SIZE],
        pub pck_ppid: sgx_key_128bit_t,
        pub tcb_cpusvn: sgx_cpu_svn_t,
        pub tcb_pce_isvsvn: sgx_isv_svn_t,
        pub pce_id: uint16_t,
        /* intel DCAP 1.7 */
        pub sgx_type: uint8_t,

        pub platform_instance_id: [uint8_t; PLATFORM_INSTANCE_ID_SIZE],
        pub dynamic_platform: pck_cert_flag_enum_t,
        pub cached_keys: pck_cert_flag_enum_t,
        pub smt_enabled: pck_cert_flag_enum_t,
    }
}

impl_struct_default! {
    sgx_ql_qv_supplemental_t; //168
}

impl_struct_ContiguousMemory! {
    sgx_ql_qv_supplemental_t;
}

/* intel DCAP 1.6 */
//
// sgx_dcap_quoteverify.h
//
impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum sgx_qv_path_type_t {
        SGX_QV_QVE_PATH  = 0,
        SGX_QV_QPL_PATH  = 1,
    }
}

/* intel sgx sdk 2.7.1 */
//
// sgx_secure_align_api.h
//
impl_struct! {
    pub struct align_req_t {
        pub offset: size_t,
        pub len: size_t,
    }
}

pub type sgx_mac_128bit_t = [uint8_t; 16];
pub type sgx_key_256bit_t = [uint8_t; 32];
pub type sgx_mac_256bit_t = [uint8_t; 32];

#[repr(C, align(32))]
#[derive(Copy, Clone, Default)]
pub struct sgx_align_key_128bit_t {
    _pad: [uint8_t; 16],
    pub key: sgx_key_128bit_t,
}

#[cfg(feature = "extra_traits")]
impl PartialEq for sgx_align_key_128bit_t {
    #[inline]
    fn eq(&self, other: &sgx_align_key_128bit_t) -> bool {
        self.key.eq(&other.key)
    }
}

#[cfg(feature = "extra_traits")]
impl Eq for sgx_align_key_128bit_t {}

#[cfg(feature = "extra_traits")]
impl core::fmt::Debug for sgx_align_key_128bit_t {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.debug_struct("sgx_align_key_128bit_t")
            .field("key", &self.key)
            .finish()
    }
}

#[repr(C, align(32))]
#[derive(Copy, Clone, Default)]
pub struct sgx_align_mac_128bit_t {
    _pad: [uint8_t; 16],
    pub mac: sgx_mac_128bit_t,
}

#[cfg(feature = "extra_traits")]
impl PartialEq for sgx_align_mac_128bit_t {
    #[inline]
    fn eq(&self, other: &sgx_align_mac_128bit_t) -> bool {
        self.mac.eq(&other.mac)
    }
}

#[cfg(feature = "extra_traits")]
impl Eq for sgx_align_mac_128bit_t {}

#[cfg(feature = "extra_traits")]
impl core::fmt::Debug for sgx_align_mac_128bit_t {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.debug_struct("sgx_align_mac_128bit_t")
            .field("mac", &self.mac)
            .finish()
    }
}

#[repr(C, align(64))]
#[derive(Copy, Clone, Default)]
pub struct sgx_align_key_256bit_t {
    _pad: [uint8_t; 8],
    pub key: sgx_key_256bit_t,
}

#[cfg(feature = "extra_traits")]
impl PartialEq for sgx_align_key_256bit_t {
    #[inline]
    fn eq(&self, other: &sgx_align_key_256bit_t) -> bool {
        self.key.eq(&other.key)
    }
}

#[cfg(feature = "extra_traits")]
impl Eq for sgx_align_key_256bit_t {}

#[cfg(feature = "extra_traits")]
impl core::fmt::Debug for sgx_align_key_256bit_t {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.debug_struct("sgx_align_key_256bit_t")
            .field("key", &self.key)
            .finish()
    }
}


#[repr(C, align(64))]
#[derive(Copy, Clone, Default)]
pub struct sgx_align_mac_256bit_t {
    _pad: [uint8_t; 8],
    pub mac: sgx_mac_256bit_t,
}

#[cfg(feature = "extra_traits")]
impl PartialEq for sgx_align_mac_256bit_t {
    #[inline]
    fn eq(&self, other: &sgx_align_mac_256bit_t) -> bool {
        self.mac.eq(&other.mac)
    }
}

#[cfg(feature = "extra_traits")]
impl Eq for sgx_align_mac_256bit_t {}

#[cfg(feature = "extra_traits")]
impl core::fmt::Debug for sgx_align_mac_256bit_t {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.debug_struct("sgx_align_mac_256bit_t")
            .field("mac", &self.mac)
            .finish()
    }
}

#[repr(C, align(64))]
#[derive(Copy, Clone, Default)]
pub struct sgx_align_ec256_dh_shared_t {
    _pad: [uint8_t; 8],
    pub key: sgx_ec256_dh_shared_t,
}

#[cfg(feature = "extra_traits")]
impl PartialEq for sgx_align_ec256_dh_shared_t  {
    #[inline]
    fn eq(&self, other: &sgx_align_ec256_dh_shared_t) -> bool {
        self.key.eq(&other.key)
    }
}

#[cfg(feature = "extra_traits")]
impl Eq for sgx_align_ec256_dh_shared_t {}

#[cfg(feature = "extra_traits")]
impl core::fmt::Debug for sgx_align_ec256_dh_shared_t {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.debug_struct("sgx_align_ec256_dh_shared_t")
            .field("key", &self.key)
            .finish()
    }
}

#[repr(C, align(64))]
#[derive(Copy, Clone, Default)]
pub struct sgx_align_ec256_private_t {
    _pad: [uint8_t; 8],
    pub key: sgx_ec256_private_t,
}

#[cfg(feature = "extra_traits")]
impl PartialEq for sgx_align_ec256_private_t  {
    #[inline]
    fn eq(&self, other: &sgx_align_ec256_private_t) -> bool {
        self.key.eq(&other.key)
    }
}

#[cfg(feature = "extra_traits")]
impl Eq for sgx_align_ec256_private_t {}

#[cfg(feature = "extra_traits")]
impl core::fmt::Debug for sgx_align_ec256_private_t {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.debug_struct("sgx_align_ec256_private_t")
            .field("key", &self.key)
            .finish()
    }
}

unsafe impl ContiguousMemory for sgx_align_key_128bit_t {}
unsafe impl ContiguousMemory for sgx_align_mac_128bit_t {}
unsafe impl ContiguousMemory for sgx_align_key_256bit_t {}
unsafe impl ContiguousMemory for sgx_align_mac_256bit_t {}
unsafe impl ContiguousMemory for sgx_align_ec256_dh_shared_t {}
unsafe impl ContiguousMemory for sgx_align_ec256_private_t {}

/* intel sgx sdk 2.8 */

//
// sgx_rsrv_mem_mngr.h
//
pub const SGX_PROT_READ: uint32_t = 0x1; /* page can be read */
pub const SGX_PROT_WRITE: uint32_t = 0x2; /* page can be written */
pub const SGX_PROT_EXEC: uint32_t = 0x4; /* page can be executed */
pub const SGX_PROT_NONE: uint32_t = 0x0; /* page can not be accessed */
