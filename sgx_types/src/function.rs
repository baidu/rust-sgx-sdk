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

use crate::metadata::*;
use crate::types::*;
use crate::*;

//#[link(name = "sgx_tstdc")]
extern "C" {
    //
    // sgx_cpuid.h
    //
    pub fn sgx_cpuid(cpuinfo: *mut [int32_t; 4], leaf: int32_t) -> sgx_status_t;
    pub fn sgx_cpuidex(cpuinfo: *mut [int32_t; 4], leaf: int32_t, subleaf: int32_t) -> sgx_status_t;

    //
    // sgx_spinlock.h
    //
    pub fn sgx_spin_lock(lock: *mut sgx_spinlock_t) -> uint32_t;
    pub fn sgx_spin_unlock(lock: *mut sgx_spinlock_t) -> uint32_t;

    //
    // sgx_thread.h
    //
    pub fn sgx_thread_mutex_init(mutex: *mut sgx_thread_mutex_t, unused: *const sgx_thread_mutex_attr_t) -> int32_t;
    pub fn sgx_thread_mutex_destroy(mutex: *mut sgx_thread_mutex_t) -> int32_t;

    pub fn sgx_thread_mutex_lock(mutex: *mut sgx_thread_mutex_t) -> int32_t;
    pub fn sgx_thread_mutex_trylock(mutex: *mut sgx_thread_mutex_t) -> int32_t;
    pub fn sgx_thread_mutex_unlock(mutex: *mut sgx_thread_mutex_t) -> int32_t;

    pub fn sgx_thread_cond_init(cond: *mut sgx_thread_cond_t, unused: *const sgx_thread_cond_attr_t) -> int32_t;
    pub fn sgx_thread_cond_destroy(cond: *mut sgx_thread_cond_t) -> int32_t;

    pub fn sgx_thread_cond_wait(cond: *mut sgx_thread_cond_t, mutex: *mut sgx_thread_mutex_t) -> int32_t;
    pub fn sgx_thread_cond_signal(cond: *mut sgx_thread_cond_t) -> int32_t;
    pub fn sgx_thread_cond_broadcast(cond: *mut sgx_thread_cond_t) -> int32_t;

    pub fn sgx_thread_self() -> sgx_thread_t;
    pub fn sgx_thread_equal(a: sgx_thread_t, b: sgx_thread_t) -> int32_t;

    /* intel sgx sdk 2.11 */
    pub fn sgx_thread_rwlock_init(rwlock: *mut sgx_thread_rwlock_t, unused: *const sgx_thread_rwlockattr_t) -> int32_t;
    pub fn sgx_thread_rwlock_destroy(rwlock: *mut sgx_thread_rwlock_t) -> int32_t;
    pub fn sgx_thread_rwlock_rdlock(rwlock: *mut sgx_thread_rwlock_t) -> int32_t;
    pub fn sgx_thread_rwlock_tryrdlock(rwlock: *mut sgx_thread_rwlock_t) -> int32_t;
    pub fn sgx_thread_rwlock_wrlock(rwlock: *mut sgx_thread_rwlock_t) -> int32_t;
    pub fn sgx_thread_rwlock_trywrlock(rwlock: *mut sgx_thread_rwlock_t) -> int32_t;
    pub fn sgx_thread_rwlock_rdunlock(rwlock: *mut sgx_thread_rwlock_t) -> int32_t;
    pub fn sgx_thread_rwlock_wrunlock(rwlock: *mut sgx_thread_rwlock_t) -> int32_t;
    pub fn sgx_thread_rwlock_unlock(rwlock: *mut sgx_thread_rwlock_t) -> int32_t;

    /* intel sgx sdk 2.8 */
    //
    // sgx_rsrv_mem_mngr.h
    //
    pub fn sgx_alloc_rsrv_mem(length: size_t) -> *mut c_void;
    pub fn sgx_free_rsrv_mem(addr: *const c_void, length: size_t) -> int32_t;
    pub fn sgx_tprotect_rsrv_mem(addr: *const c_void, length: size_t, prot: int32_t) -> sgx_status_t;

    /* intel sgx sdk 2.11 */
    pub fn sgx_get_rsrv_mem_info(start_addr: *mut *mut c_void, max_size: *mut size_t) -> sgx_status_t;
    pub fn sgx_alloc_rsrv_mem_ex(desired_addr: *const c_void, length: size_t) -> *mut c_void;
}

//#[link(name = "sgx_tservice")]
extern "C" {
    //
    // sgx_dh.h
    //
    pub fn sgx_dh_init_session(role: sgx_dh_session_role_t, session: *mut sgx_dh_session_t) -> sgx_status_t;

    pub fn sgx_dh_responder_gen_msg1(msg1: *mut sgx_dh_msg1_t,
                                     dh_session: *mut sgx_dh_session_t) -> sgx_status_t;

    pub fn sgx_dh_initiator_proc_msg1(msg1: *const sgx_dh_msg1_t,
                                      msg2: *mut sgx_dh_msg2_t,
                                      dh_session: *mut sgx_dh_session_t) -> sgx_status_t;

    pub fn sgx_dh_responder_proc_msg2(msg2: *const sgx_dh_msg2_t,
                                      msg3: *mut sgx_dh_msg3_t,
                                      dh_session: *mut sgx_dh_session_t,
                                      aek: *mut sgx_key_128bit_t,
                                      initiator_identity: *mut sgx_dh_session_enclave_identity_t) -> sgx_status_t;

    pub fn sgx_dh_initiator_proc_msg3(msg3: *const sgx_dh_msg3_t,
                                      dh_session: *mut sgx_dh_session_t,
                                      aek: *mut sgx_key_128bit_t,
                                      responder_identity: *mut sgx_dh_session_enclave_identity_t) -> sgx_status_t;

    //
    // sgx_tae_service.h
    //
    /* delete intel sgx sdk 2.8 */
    /*
    pub fn sgx_create_pse_session() -> sgx_status_t;
    pub fn sgx_close_pse_session() -> sgx_status_t;
    pub fn sgx_get_ps_sec_prop(security_property: *mut sgx_ps_sec_prop_desc_t) -> sgx_status_t;
    // intel sgx sdk 1.8 
    pub fn sgx_get_ps_sec_prop_ex(security_property: *mut sgx_ps_sec_prop_desc_ex_t) -> sgx_status_t;
    pub fn sgx_get_trusted_time(current_time: *mut sgx_time_t, time_source_nonce: *mut sgx_time_source_nonce_t) -> sgx_status_t;

    pub fn sgx_create_monotonic_counter_ex(owner_policy: uint16_t,
                                           owner_attribute_mask: *const sgx_attributes_t,
                                           counter_uuid: *mut sgx_mc_uuid_t,
                                           counter_value: *mut uint32_t) -> sgx_status_t;

    pub fn sgx_create_monotonic_counter(counter_uuid: *mut sgx_mc_uuid_t, counter_value: *mut uint32_t) -> sgx_status_t;
    pub fn sgx_destroy_monotonic_counter(counter_uuid: *const sgx_mc_uuid_t) -> sgx_status_t;
    pub fn sgx_increment_monotonic_counter(counter_uuid: *const sgx_mc_uuid_t, counter_value: *mut uint32_t) -> sgx_status_t;
    pub fn sgx_read_monotonic_counter(counter_uuid: *const sgx_mc_uuid_t, counter_value: *mut uint32_t) -> sgx_status_t;
    */

    //
    // sgx_tseal.h
    //
    pub fn sgx_calc_sealed_data_size(add_mac_txt_size: uint32_t, txt_encrypt_size: uint32_t) -> uint32_t;
    pub fn sgx_get_add_mac_txt_len(p_sealed_data: *const sgx_sealed_data_t) -> uint32_t;
    pub fn sgx_get_encrypt_txt_len(p_sealed_data: *const sgx_sealed_data_t) -> uint32_t;

    pub fn sgx_seal_data(additional_MACtext_length: uint32_t,
                         p_additional_MACtext: *const uint8_t,
                         text2encrypt_length: uint32_t,
                         p_text2encrypt: *const uint8_t,
                         sealed_data_size: uint32_t,
                         p_sealed_data: *mut sgx_sealed_data_t) -> sgx_status_t;

    pub fn sgx_seal_data_ex(key_policy: uint16_t,
                            attribute_mask: sgx_attributes_t,
                            misc_mask: sgx_misc_select_t,
                            additional_MACtext_length: uint32_t,
                            p_additional_MACtext: *const uint8_t,
                            text2encrypt_length: uint32_t,
                            p_text2encrypt: *const uint8_t,
                            sealed_data_size: uint32_t,
                            p_sealed_data: *mut sgx_sealed_data_t) -> sgx_status_t;

    pub fn sgx_unseal_data(p_sealed_data: *const sgx_sealed_data_t,
                           p_additional_MACtext: *mut uint8_t,
                           p_additional_MACtext_length: *mut uint32_t,
                           p_decrypted_text: *mut uint8_t,
                           p_decrypted_text_length: *mut uint32_t) -> sgx_status_t;

    pub fn sgx_mac_aadata(additional_MACtext_length: uint32_t,
                          p_additional_MACtext: *const uint8_t,
                          sealed_data_size: uint32_t,
                          p_sealed_data: *mut sgx_sealed_data_t) -> sgx_status_t;

    pub fn sgx_mac_aadata_ex(key_policy: uint16_t,
                             attribute_mask: sgx_attributes_t,
                             misc_mask: sgx_misc_select_t,
                             additional_MACtext_length: uint32_t,
                             p_additional_MACtext: *const uint8_t,
                             sealed_data_size: uint32_t,
                             p_sealed_data: *mut sgx_sealed_data_t) -> sgx_status_t;

    pub fn sgx_unmac_aadata(p_sealed_data: *const sgx_sealed_data_t,
                            p_additional_MACtext: *mut uint8_t,
                            p_additional_MACtext_length: *mut uint32_t) -> sgx_status_t;


    //
    // sgx_utils.h
    //
    pub fn sgx_create_report(target_info : *const sgx_target_info_t,
                             report_data: *const sgx_report_data_t,
                             report: *mut sgx_report_t) -> sgx_status_t;
    /* intel sgx sdk 2.4 */
    pub fn sgx_self_report() -> *const sgx_report_t;
    pub fn sgx_self_target(target_info: *mut sgx_target_info_t) -> sgx_status_t;

    pub fn sgx_verify_report(report: *const sgx_report_t) -> sgx_status_t;
    pub fn sgx_get_key(key_request: *const sgx_key_request_t, key: *mut sgx_key_128bit_t) -> sgx_status_t;

    /* intel sgx sdk 2.7.1 */
    //
    // sgx_secure_align_api.h
    //
    pub fn sgx_aligned_malloc(size: size_t, alignment: size_t, data: *const align_req_t, count: size_t) -> *mut c_void;
    pub fn sgx_aligned_free(ptr: *mut c_void);
    pub fn sgx_get_aligned_ptr(raw: *mut c_void, raw_size: size_t, allocate_size: size_t, alignment: size_t, data: *const align_req_t, count: size_t) -> *mut c_void;
}

//#[link(name = "sgx_tcrypto")]
extern "C" {
    //
    // sgx_tcrypto.h
    //
    pub fn sgx_sha256_msg(p_src: *const uint8_t, src_len: uint32_t, p_hash: *mut sgx_sha256_hash_t) -> sgx_status_t;
    pub fn sgx_sha256_init(p_sha_handle: *mut sgx_sha_state_handle_t) -> sgx_status_t;
    pub fn sgx_sha256_update(p_src: *const uint8_t, src_len: uint32_t, sha_handle: sgx_sha_state_handle_t) -> sgx_status_t;
    pub fn sgx_sha256_get_hash(sha_handle: sgx_sha_state_handle_t, p_hash: *mut sgx_sha256_hash_t) -> sgx_status_t;
    pub fn sgx_sha256_close(sha_handle: sgx_sha_state_handle_t) -> sgx_status_t;

    /* instel sgx sdk 2.4 */
    pub fn sgx_sha1_msg(p_src: *const uint8_t, src_len: uint32_t, p_hash: *mut sgx_sha1_hash_t) -> sgx_status_t;
    pub fn sgx_sha1_init(p_sha_handle: *mut sgx_sha_state_handle_t) -> sgx_status_t;
    pub fn sgx_sha1_update(p_src: *const uint8_t, src_len: uint32_t, sha_handle: sgx_sha_state_handle_t) -> sgx_status_t;
    pub fn sgx_sha1_get_hash(sha_handle: sgx_sha_state_handle_t, p_hash: *mut sgx_sha1_hash_t) -> sgx_status_t;
    pub fn sgx_sha1_close(sha_handle: sgx_sha_state_handle_t) -> sgx_status_t;

    pub fn sgx_rijndael128GCM_encrypt(p_key: *const sgx_aes_gcm_128bit_key_t,
                                      p_src: *const uint8_t,
                                      src_len: uint32_t,
                                      p_dst: *mut uint8_t,
                                      p_iv: *const uint8_t,
                                      iv_len: uint32_t,
                                      p_aad: *const uint8_t,
                                      aad_len: uint32_t,
                                      p_out_mac: *mut sgx_aes_gcm_128bit_tag_t) -> sgx_status_t;

    pub fn sgx_rijndael128GCM_decrypt(p_key: *const sgx_aes_gcm_128bit_key_t,
                                      p_src: *const uint8_t,
                                      src_len: uint32_t,
                                      p_dst: *mut uint8_t,
                                      p_iv: *const uint8_t,
                                      iv_len: uint32_t,
                                      p_aad: *const uint8_t,
                                      aad_len: uint32_t,
                                      p_in_mac: *const sgx_aes_gcm_128bit_tag_t) -> sgx_status_t;

    pub fn sgx_rijndael128_cmac_msg(p_key: *const sgx_cmac_128bit_key_t, p_src: *const uint8_t, src_len: uint32_t, p_mac: *mut sgx_cmac_128bit_tag_t) -> sgx_status_t;
    pub fn sgx_cmac128_init(p_key: *const sgx_cmac_128bit_key_t, p_cmac_handle: *mut sgx_cmac_state_handle_t) -> sgx_status_t;
    pub fn sgx_cmac128_update(p_src: *const uint8_t, src_len: uint32_t, cmac_handle: sgx_cmac_state_handle_t) -> sgx_status_t;
    pub fn sgx_cmac128_final(cmac_handle: sgx_cmac_state_handle_t, p_hash: *mut sgx_cmac_128bit_tag_t) -> sgx_status_t;
    pub fn sgx_cmac128_close(cmac_handle: sgx_cmac_state_handle_t) -> sgx_status_t;

    /* intel sgx sdk 2.4 */
    pub fn sgx_hmac_sha256_msg(p_src: *const uint8_t,
                               src_len: int32_t,
                               p_key: *const uint8_t,
                               key_len: int32_t,
                               p_mac: *mut uint8_t,
                               mac_len: int32_t) -> sgx_status_t;

    pub fn sgx_hmac256_init(p_key: *const uint8_t, key_len: int32_t, p_hmac_handle: *mut sgx_hmac_state_handle_t) -> sgx_status_t;
    pub fn sgx_hmac256_update(p_src: *const uint8_t, src_len: int32_t, hmac_handle: sgx_hmac_state_handle_t) -> sgx_status_t;
    pub fn sgx_hmac256_final(p_hash: *mut uint8_t, hash_len: int32_t, hmac_handle: sgx_hmac_state_handle_t) -> sgx_status_t;
    pub fn sgx_hmac256_close(hmac_handle: sgx_hmac_state_handle_t) -> sgx_status_t;

    pub fn sgx_aes_ctr_encrypt(p_key: *const sgx_aes_ctr_128bit_key_t,
                               p_src: *const uint8_t,
                               src_len: uint32_t,
                               p_ctr: *mut uint8_t,
                               ctr_inc_bits: uint32_t,
                               p_dst: *mut uint8_t) -> sgx_status_t;

    pub fn sgx_aes_ctr_decrypt(p_key: *const sgx_aes_ctr_128bit_key_t,
                               p_src: *const uint8_t,
                               src_len: uint32_t,
                               p_ctr: *mut uint8_t,
                               ctr_inc_bits: uint32_t,
                               p_dst: *mut uint8_t) -> sgx_status_t;

    pub fn sgx_ecc256_open_context(p_ecc_handle: *mut sgx_ecc_state_handle_t) -> sgx_status_t;
    pub fn sgx_ecc256_close_context(ecc_handle: sgx_ecc_state_handle_t) -> sgx_status_t;

    pub fn sgx_ecc256_create_key_pair(p_private: *mut sgx_ec256_private_t, p_public: *mut sgx_ec256_public_t, ecc_handle: sgx_ecc_state_handle_t) -> sgx_status_t;
    pub fn sgx_ecc256_check_point(p_point: *const sgx_ec256_public_t, ecc_handle: sgx_ecc_state_handle_t, p_valid: *mut int32_t) -> sgx_status_t;

    pub fn sgx_ecc256_compute_shared_dhkey(p_private_b: *const sgx_ec256_private_t,
                                           p_public_ga: *const sgx_ec256_public_t,
                                           p_shared_key: *mut sgx_ec256_dh_shared_t,
                                           ecc_handle: sgx_ecc_state_handle_t) -> sgx_status_t;
    /* intel sgx sdk 1.8 */
    /* delete (intel sgx sdk 2.0)
    pub fn sgx_ecc256_compute_shared_dhkey512(p_private_b: *mut sgx_ec256_private_t,
                                              p_public_ga: *mut sgx_ec256_public_t,
                                              p_shared_key: *mut sgx_ec256_dh_shared512_t,
                                              ecc_handle: sgx_ecc_state_handle_t) -> sgx_status_t;
    */

    pub fn sgx_ecdsa_sign(p_data: *const uint8_t,
                          data_size: uint32_t,
                          p_private: *const sgx_ec256_private_t,
                          p_signature: *mut sgx_ec256_signature_t,
                          ecc_handle: sgx_ecc_state_handle_t) -> sgx_status_t;

    pub fn sgx_ecdsa_verify(p_data: *const uint8_t,
                            data_size:  uint32_t,
                            p_public: *const sgx_ec256_public_t,
                            p_signature: *const sgx_ec256_signature_t,
                            p_result: *mut uint8_t,
                            ecc_handle: sgx_ecc_state_handle_t) -> sgx_status_t;

    /* intel sgx sdk 2.4 */
    pub fn sgx_ecdsa_verify_hash(hash: *const uint8_t,
                                 p_public: *const sgx_ec256_public_t,
                                 p_signature: *const sgx_ec256_signature_t,
                                 p_result: *mut uint8_t,
                                 ecc_handle: sgx_ecc_state_handle_t) -> sgx_status_t;

    /* intel sgx sdk 1.9 */
    /*
    pub fn sgx_rsa3072_sign(p_data: *const uint8_t,
                            data_size: uint32_t,
                            p_private: *const sgx_rsa3072_private_key_t,
                            p_signature: *mut sgx_rsa3072_signature_t) -> sgx_status_t;
    */

    /* intel sgx sdk 2.0 */
    pub fn sgx_rsa3072_sign(p_data: *const uint8_t,
                            data_size: uint32_t,
                            p_key: *const sgx_rsa3072_key_t,
                            p_signature: *mut sgx_rsa3072_signature_t) -> sgx_status_t;

    pub fn sgx_rsa3072_verify(p_data: *const uint8_t,
                              data_size: uint32_t,
                              p_public: *const sgx_rsa3072_public_key_t,
                              p_signature: *const sgx_rsa3072_signature_t,
                              p_result: *mut sgx_rsa_result_t) -> sgx_status_t;

    /* intel sgx sdk 2.1.3 */
    pub fn sgx_create_rsa_key_pair(n_byte_size: int32_t,
                                   e_byte_size: int32_t,
                                   p_n: *mut uint8_t,
                                   p_d: *mut uint8_t,
                                   p_e: *mut uint8_t,
                                   p_p: *mut uint8_t,
                                   p_q: *mut uint8_t,
                                   p_dmp1: *mut uint8_t,
                                   p_dmq1: *mut uint8_t,
                                   p_iqmp: *mut uint8_t) -> sgx_status_t;

    pub fn sgx_rsa_priv_decrypt_sha256(rsa_key: *const c_void,
                                       pout_data: *mut uint8_t,
                                       pout_len: *mut size_t,
                                       pin_data: *const uint8_t,
                                       pin_len: size_t) -> sgx_status_t;

    pub fn sgx_rsa_pub_encrypt_sha256(rsa_key: *const c_void,
                                      pout_data: *mut uint8_t,
                                      pout_len: *mut size_t,
                                      pin_data: *const uint8_t,
                                      pin_len: size_t) -> sgx_status_t;

    pub fn sgx_create_rsa_priv2_key(mod_size: int32_t,
                                    exp_size: int32_t,
                                    p_rsa_key_e: *const uint8_t,
                                    p_rsa_key_p: *const uint8_t,
                                    p_rsa_key_q: *const uint8_t,
                                    p_rsa_key_dmp1: *const uint8_t,
                                    p_rsa_key_dmq1: *const uint8_t,
                                    p_rsa_key_iqmp: *const uint8_t,
                                    new_pri_key2: *mut *mut c_void) -> sgx_status_t;


    /* intel sgx sdk 2.6 */
    pub fn sgx_create_rsa_priv1_key(n_byte_size: int32_t,
                                    e_byte_size: int32_t,
                                    d_byte_size: int32_t,
                                    le_n: *const uint8_t,
                                    le_e: *const uint8_t,
                                    le_d: *const uint8_t,
                                    new_pri_key1: *mut *mut c_void) -> sgx_status_t;

    pub fn sgx_create_rsa_pub1_key(mod_size: int32_t,
                                   exp_size: int32_t,
                                   le_n: *const uint8_t,
                                   le_e: *const uint8_t,
                                   new_pub_key1: *mut *mut c_void) -> sgx_status_t;

    pub fn sgx_free_rsa_key(p_rsa_key: *const c_void,
                            key_type: sgx_rsa_key_type_t,
                            mod_size: int32_t,
                            exp_size: int32_t) -> sgx_status_t;

    pub fn sgx_calculate_ecdsa_priv_key(hash_drg: *const uint8_t,
                                        hash_drg_len: int32_t,
                                        sgx_nistp256_r_m1: *const uint8_t,
                                        sgx_nistp256_r_m1_len: int32_t,
                                        out_key: *mut uint8_t,
                                        out_key_len: int32_t) -> sgx_status_t;

    /* intel sgx sdk 2.3 */
    pub fn sgx_ecc256_calculate_pub_from_priv(p_att_priv_key: *const sgx_ec256_private_t,
                                              p_att_pub_key: *mut sgx_ec256_public_t) -> sgx_status_t;


    /* intel sgx sdk 2.4 */
    pub fn sgx_aes_gcm128_enc_init(p_key: *const uint8_t,
                                   p_iv: *const uint8_t,
                                   iv_len: uint32_t,
                                   p_aad: *const uint8_t,
                                   aad_len: uint32_t,
                                   aes_gcm_state: *mut sgx_aes_state_handle_t) -> sgx_status_t;
    pub fn sgx_aes_gcm128_enc_get_mac(mac: *mut uint8_t, aes_gcm_state: sgx_aes_state_handle_t) -> sgx_status_t;
    pub fn sgx_aes_gcm_close(aes_gcm_state: sgx_aes_state_handle_t) -> sgx_status_t;
    pub fn sgx_aes_gcm128_enc_update(p_src: *const uint8_t,
                                     src_len: uint32_t,
                                     p_dst: *mut uint8_t,
                                     aes_gcm_state: sgx_aes_state_handle_t) -> sgx_status_t;
}

//#[link(name = "sgx_tkey_exchange")]
extern "C" {
    //
    // sgx_tkey_exchange.h
    //
    pub fn sgx_ra_init(p_pub_key: *const sgx_ec256_public_t, b_pse: int32_t, p_context: *mut sgx_ra_context_t) -> sgx_status_t;
    pub fn sgx_ra_init_ex(p_pub_key: *const sgx_ec256_public_t,
                          b_pse: int32_t,
                          derive_key_cb: sgx_ra_derive_secret_keys_t,
                          p_context: *mut sgx_ra_context_t) -> sgx_status_t;
    pub fn sgx_ra_get_keys(context: sgx_ra_context_t,
                           keytype: sgx_ra_key_type_t,
                           p_key: *mut sgx_ra_key_128_t) -> sgx_status_t;
    pub fn sgx_ra_close(context: sgx_ra_context_t) -> sgx_status_t;
    pub fn sgx_ra_get_ga(eid: sgx_enclave_id_t, retval: *mut sgx_status_t,
                         context: sgx_ra_context_t, g_a: *mut sgx_ec256_public_t) -> sgx_status_t;
}

//#[link(name = "sgx_trts")]
extern "C" {
    //
    // sgx_trts.h
    //
    pub fn sgx_is_within_enclave(addr: *const c_void, size: size_t) -> int32_t;
    pub fn sgx_is_outside_enclave(addr: *const c_void, size: size_t) -> int32_t;
    pub fn sgx_read_rand(rand: *mut u8, length_in_bytes: size_t) -> sgx_status_t;
    /* intel sgx sdk 2.1.2 */
    pub fn sgx_is_enclave_crashed() -> int32_t;

    //
    // sgx_trts_exception.h
    //
    pub fn sgx_register_exception_handler(is_first_handler: uint32_t,
                                          exception_handler: sgx_exception_handler_t) -> *const c_void;
    pub fn sgx_unregister_exception_handler(handler: *const c_void) -> uint32_t;

    //
    // sgx_edger8r.h
    //
    pub fn sgx_ocalloc(size: size_t) -> *mut c_void;
    pub fn sgx_ocfree();
}

//#[link(name = "sgx_epid")]
extern "C" {
    //
    // sgx_uae_epid.h
    //
    pub fn sgx_init_quote(p_target_info: *mut sgx_target_info_t, p_gid: *mut sgx_epid_group_id_t) -> sgx_status_t;

    /* intel sgx sdk 1.9 */
    pub fn sgx_calc_quote_size(p_sig_rl: *const uint8_t, sig_rl_size: uint32_t, p_quote_size: *mut uint32_t) -> sgx_status_t;
    pub fn sgx_get_quote_size(p_sig_rl: *const uint8_t, p_quote_size: *mut uint32_t) -> sgx_status_t;

    pub fn sgx_get_quote(p_report: *const sgx_report_t,
                         quote_type: sgx_quote_sign_type_t,
                         p_spid: *const sgx_spid_t,
                         p_nonce: *const sgx_quote_nonce_t,
                         p_sig_rl: *const uint8_t,
                         sig_rl_size: uint32_t,
                         p_qe_report: *mut sgx_report_t,
                         p_quote: *mut sgx_quote_t,
                         quote_size: uint32_t) -> sgx_status_t;

    pub fn sgx_get_extended_epid_group_id(p_extended_epid_group_id: *mut uint32_t) -> sgx_status_t;
    pub fn sgx_report_attestation_status(p_platform_info: *const sgx_platform_info_t,
                                         attestation_status: int32_t,
                                         p_update_info: *mut sgx_update_info_bit_t) -> sgx_status_t;

    /* intel sgx sdk 2.6 */
    pub fn sgx_check_update_status(p_platform_info: *const sgx_platform_info_t,
                                   p_update_info: *mut sgx_update_info_bit_t,
                                   config: uint32_t,
                                   p_status: *mut uint32_t) -> sgx_status_t;
}

//#[link(name = "sgx_launch")]
extern "C" {
    //
    // sgx_uae_launch.h
    //

    pub fn sgx_get_whitelist_size(p_whitelist_size: *mut uint32_t) -> sgx_status_t;
    pub fn sgx_get_whitelist(p_whitelist: *mut uint8_t, whitelist_size: uint32_t) -> sgx_status_t;

    /* intel sgx sdk 2.1 */
    pub fn sgx_register_wl_cert_chain(p_wl_cert_chain: *const uint8_t,
                                      wl_cert_chain_size: uint32_t) -> sgx_status_t;
}

//#[link(name = "sgx_platform")]
extern "C" {
    //
    // sgx_uae_platform.h
    //
    pub fn sgx_get_ps_cap(p_sgx_ps_cap: *mut sgx_ps_cap_t) -> sgx_status_t;
}

//#[link(name = "sgx_quote_ex")]
extern "C" {
    //
    // sgx_uae_quote_ex.h
    //

    /* intel sgx sdk 2.5 */
    pub fn sgx_select_att_key_id(p_att_key_id_list: *const uint8_t,
                                 att_key_id_list_size: uint32_t,
                                 pp_selected_key_id: *mut sgx_att_key_id_t) -> sgx_status_t;

    pub fn sgx_init_quote_ex(p_att_key_id: *const sgx_att_key_id_t,
                             p_qe_target_info: *mut sgx_target_info_t,
                             p_pub_key_id_size: *mut size_t,
                             p_pub_key_id: *mut uint8_t) -> sgx_status_t;

    pub fn sgx_get_quote_size_ex(p_att_key_id: *const sgx_att_key_id_t, p_quote_size: *mut uint32_t) -> sgx_status_t;

    pub fn sgx_get_quote_ex(p_app_report: *const sgx_report_t,
                            p_att_key_id: *const sgx_att_key_id_t,
                            p_qe_report_info: *mut sgx_qe_report_info_t,
                            p_quote: *mut uint8_t,
                            quote_size: uint32_t) -> sgx_status_t;

     /* intel sgx sdk 2.9.1 */
     pub fn sgx_get_supported_att_key_id_num(p_att_key_id_num: *mut uint32_t) -> sgx_status_t;
     pub fn sgx_get_supported_att_key_ids(p_att_key_id_list: *mut sgx_att_key_id_ext_t,
                                          att_key_id_num: uint32_t) -> sgx_status_t;
}

//#[link(name = "sgx_uae_service")]
extern "C" {
    //
    // sgx_uae_service.h
    //

    // intel sgx sdk 2.7
    // Split libsgx_uae_service.so to libsgx_epid.so, libsgx_launch.so, libsgx_platform.so and libsgx_quote_ex.so.
    //
}

//#[link(name = "sgx_ukey_exchange")]
extern "C" {
    //
    // sgx_ukey_exchange.h
    //
    pub fn sgx_ra_get_msg1(context: sgx_ra_context_t,
                           eid: sgx_enclave_id_t,
                           p_get_ga: sgx_ecall_get_ga_trusted_t,
                           p_msg1: *mut sgx_ra_msg1_t) -> sgx_status_t;

    pub fn sgx_ra_proc_msg2(context: sgx_ra_context_t,
                            eid: sgx_enclave_id_t,
                            p_proc_msg2: sgx_ecall_proc_msg2_trusted_t,
                            p_get_msg3: sgx_ecall_get_msg3_trusted_t,
                            p_msg2: *const sgx_ra_msg2_t,
                            msg2_size: uint32_t,
                            pp_msg3: *mut *mut sgx_ra_msg3_t,
                            p_msg3_size: *mut uint32_t) -> sgx_status_t;

    /* intel sgx sdk 2.5 */
    pub fn sgx_ra_get_msg1_ex(p_att_key_id: *const sgx_att_key_id_t,
                              context: sgx_ra_context_t,
                              eid: sgx_enclave_id_t,
                              p_get_ga: sgx_ecall_get_ga_trusted_t,
                              p_msg1: *mut sgx_ra_msg1_t) -> sgx_status_t;

    pub fn sgx_ra_proc_msg2_ex(p_att_key_id: *const sgx_att_key_id_t,
                               context: sgx_ra_context_t,
                               eid: sgx_enclave_id_t,
                               p_proc_msg2: sgx_ecall_proc_msg2_trusted_t,
                               p_get_msg3: sgx_ecall_get_msg3_trusted_t,
                               p_msg2: *const sgx_ra_msg2_t,
                               msg2_size: uint32_t,
                               pp_msg3: *mut *mut sgx_ra_msg3_t,
                               p_msg3_size: *mut uint32_t) -> sgx_status_t;
}


//#[link(name = "sgx_urts")]
extern "C" {
    //
    // sgx_urts.h
    //
    pub fn sgx_create_enclave(file_name: *const c_char,
                              debug: int32_t,
                              launch_token: *mut sgx_launch_token_t,
                              launch_token_updated: *mut int32_t,
                              enclave_id: *mut sgx_enclave_id_t,
                              misc_attr: *mut sgx_misc_attribute_t) -> sgx_status_t;

    /* intel sgx sdk 2.1.3 */
    pub fn sgx_create_encrypted_enclave(file_name: *const c_char,
                                        debug: int32_t,
                                        launch_token: *mut sgx_launch_token_t,
                                        launch_token_updated: *mut int32_t,
                                        enclave_id: *mut sgx_enclave_id_t,
                                        misc_attr: *mut sgx_misc_attribute_t,
                                        sealed_key: *const uint8_t) -> sgx_status_t;

    /* intel sgx sdk 2.2 */
    pub fn sgx_create_enclave_ex(file_name: *const c_char,
                                 debug: int32_t,
                                 launch_token: *mut sgx_launch_token_t,
                                 launch_token_updated: *mut int32_t,
                                 enclave_id: *mut sgx_enclave_id_t,
                                 misc_attr: *mut sgx_misc_attribute_t,
                                 ex_features: uint32_t,
                                 ex_features_p: *const [*const c_void; 32]) -> sgx_status_t;

    /* intel sgx sdk 2.4 */
    pub fn sgx_create_enclave_from_buffer_ex(buffer: *const uint8_t,
                                             buffer_size: size_t,
                                             debug: int32_t,
                                             enclave_id: *mut sgx_enclave_id_t,
                                             misc_attr: *mut sgx_misc_attribute_t,
                                             ex_features: uint32_t,
                                             ex_features_p: *const [*const c_void; 32]) -> sgx_status_t;

    pub fn sgx_destroy_enclave(enclave_id: sgx_enclave_id_t) -> sgx_status_t;

    /* intel sgx sdk 2.4 */
    pub fn sgx_get_target_info(enclave_id: sgx_enclave_id_t, target_info: *mut sgx_target_info_t) -> sgx_status_t;

    /* intel sgx sdk 2.9.1 */
    pub fn sgx_get_metadata(enclave_file: *const c_char, metadata: *mut metadata_t) -> sgx_status_t;
}

/* intel sgx sdk 1.9 */
//#[link(name = "sgx_tprotected_fs")]
extern "C" {
    //
    // sgx_tprotected_fs.h
    //
    pub fn sgx_fopen(filename: *const c_char,
                     mode: *const c_char,
                     key: *const sgx_key_128bit_t) -> SGX_FILE;

    pub fn sgx_fopen_auto_key(filename: *const c_char, mode: *const c_char) -> SGX_FILE;

    pub fn sgx_fwrite(ptr: *const c_void,
                      size: size_t,
                      count: size_t,
                      stream: SGX_FILE) -> size_t;

    pub fn sgx_fread(ptr: *mut c_void,
                     size: size_t,
                     count: size_t,
                     stream: SGX_FILE) -> size_t;

    pub fn sgx_ftell(stream: SGX_FILE) -> int64_t;
    pub fn sgx_fseek(stream: SGX_FILE, offset: int64_t, origin: int32_t) -> int32_t;
    pub fn sgx_fflush(stream: SGX_FILE) -> int32_t;
    pub fn sgx_ferror(stream: SGX_FILE) -> int32_t;
    pub fn sgx_feof(stream: SGX_FILE) -> int32_t;
    pub fn sgx_clearerr(stream: SGX_FILE);
    pub fn sgx_fclose(stream: SGX_FILE) -> int32_t;
    pub fn sgx_remove(filename: *const c_char) -> int32_t;
    pub fn sgx_fexport_auto_key(filename: *const c_char, key: *mut sgx_key_128bit_t) -> int32_t;
    pub fn sgx_fimport_auto_key(filename: *const c_char, key: *const sgx_key_128bit_t) -> int32_t;
    pub fn sgx_fclear_cache(stream: SGX_FILE) -> int32_t;
}

/* intel sgx sdk 2.0 */
//#[link(name = "sgx_capable")]
extern "C" {
    //
    // sgx_capable.h
    //
    pub fn sgx_is_capable(sgx_capable: *mut int32_t) -> sgx_status_t;
    pub fn sgx_cap_enable_device(sgx_device_status: *mut sgx_device_status_t) -> sgx_status_t;
    pub fn sgx_cap_get_status(sgx_device_status: *mut sgx_device_status_t) -> sgx_status_t;
}

//#[link(name = "sgx_pce_wrapper")]
extern "C" {
    //
    // sgx_pce.h
    //
    pub fn sgx_set_pce_enclave_load_policy(policy: sgx_ql_request_policy_t) -> sgx_pce_error_t;
    pub fn sgx_pce_get_target(p_pce_target: *mut sgx_target_info_t, p_pce_isv_svn: *mut sgx_isv_svn_t) -> sgx_pce_error_t;
    pub fn sgx_get_pce_info(p_report: *const sgx_report_t,
                            p_public_key: *const uint8_t,
                            key_size: uint32_t,
                            crypto_suite: uint8_t,
                            p_encrypted_ppid: *mut uint8_t,
                            encrypted_ppid_buf_size: uint32_t,
                            p_encrypted_ppid_out_size: *mut uint32_t,
                            p_pce_isv_svn: *mut sgx_isv_svn_t,
                            p_pce_id: *mut uint16_t,
                            p_signature_scheme: *mut uint8_t) -> sgx_pce_error_t;
    pub fn sgx_pce_sign_report(isv_svn: *const sgx_isv_svn_t,
                               cpu_svn: *const sgx_cpu_svn_t,
                               p_report: *const sgx_report_t,
                               p_signature: *mut uint8_t,
                               signature_buf_size: uint32_t,
                               p_signature_out_size: *mut uint32_t) -> sgx_pce_error_t;

    /* intel DCAP 1.5 */
    pub fn sgx_get_pce_info_without_ppid(p_pce_isvsvn: *mut sgx_isv_svn_t,
                                         p_pce_id: *mut uint16_t) -> sgx_pce_error_t;
}

//#[link(name = "sgx_dcap_ql")]
extern "C" {
    //
    // sgx_dcap_ql_wrapper.h
    //
    pub fn sgx_qe_set_enclave_load_policy(policy: sgx_ql_request_policy_t) -> sgx_quote3_error_t;
    pub fn sgx_qe_get_target_info(p_qe_target_info: *mut sgx_target_info_t) -> sgx_quote3_error_t;
    pub fn sgx_qe_get_quote_size(p_quote_size: *mut uint32_t) -> sgx_quote3_error_t;
    pub fn sgx_qe_get_quote(p_app_report: *const sgx_report_t, quote_size: uint32_t, p_quote: *mut uint8_t) -> sgx_quote3_error_t;
    pub fn sgx_qe_cleanup_by_policy() -> sgx_quote3_error_t;

    /* intel DCAP 1.6 */
    pub fn sgx_ql_set_path(path_type: sgx_ql_path_type_t, p_path: *const c_char) -> sgx_quote3_error_t;
}

//#[link(name = "dcap_quoteprov")]
extern "C" {
    //
    // sgx_default_quote_provider.h
    //
    pub fn sgx_ql_get_quote_config(p_pck_cert_id: *const sgx_ql_pck_cert_id_t, pp_quote_config: *mut *mut sgx_ql_config_t) -> sgx_quote3_error_t;
    pub fn sgx_ql_free_quote_config(p_quote_config: *const sgx_ql_config_t) -> sgx_quote3_error_t;
    pub fn sgx_ql_get_quote_verification_collateral(fmspc: *const uint8_t,
                                                    fmspc_size: uint16_t,
                                                    pck_ra: *const c_char,
                                                    pp_quote_collateral: *mut *mut sgx_ql_qve_collateral_t) -> sgx_quote3_error_t;
    pub fn sgx_ql_free_quote_verification_collateral(p_quote_collateral: *const sgx_ql_qve_collateral_t) -> sgx_quote3_error_t;
    pub fn sgx_ql_get_qve_identity(pp_qve_identity: *mut *mut c_char,
                                   p_qve_identity_size: *mut uint32_t,
                                   pp_qve_identity_issuer_chain: *mut *mut c_char,
                                   p_qve_identity_issuer_chain_size: *mut uint32_t) -> sgx_quote3_error_t;
    pub fn sgx_ql_free_qve_identity(p_qve_identity: *const c_char, p_qve_identity_issuer_chain: *const c_char) -> sgx_quote3_error_t;

    /* intel DCAP 1.4 */
    pub fn sgx_ql_get_root_ca_crl(pp_root_ca_crl: *mut *mut uint8_t, p_root_ca_cal_size: *mut uint16_t) -> sgx_quote3_error_t;
    pub fn sgx_ql_free_root_ca_crl(p_root_ca_crl: *const uint8_t) -> sgx_quote3_error_t;
    /* intel DCAP 2.14 */
    pub fn sgx_ql_set_logging_callback(logger: sgx_ql_logging_callback_t) -> sgx_quote3_error_t;
}

//#[link(name = "sgx_default_qcnl_wrapper")]
extern "C" {
    //
    // sgx_default_qcnl_wrapper.h
    //
    pub fn sgx_qcnl_get_pck_cert_chain(p_pck_cert_id: *const sgx_ql_pck_cert_id_t, pp_quote_config: *mut *mut sgx_ql_config_t) -> sgx_qcnl_error_t;
    pub fn sgx_qcnl_free_pck_cert_chain(p_quote_config: *const sgx_ql_config_t);
    pub fn sgx_qcnl_get_pck_crl_chain(ca: *const c_char, ca_size: uint16_t, p_crl_chain: *mut *mut uint8_t, p_crl_chain_size: *mut uint16_t) -> sgx_qcnl_error_t;
    pub fn sgx_qcnl_free_pck_crl_chain(p_crl_chain: *const uint8_t);
    pub fn sgx_qcnl_get_tcbinfo(fmspc: *const c_char, fmspc_size: uint16_t, p_tcbinfo: *mut *mut uint8_t, p_tcbinfo_size: *mut uint16_t) -> sgx_qcnl_error_t;
    pub fn sgx_qcnl_free_tcbinfo(p_tcbinfo: *const uint8_t);
    pub fn sgx_qcnl_get_qe_identity(qe_type: uint8_t, p_qe_identity: *mut *mut uint8_t, p_qe_identity_size: *mut uint16_t) -> sgx_qcnl_error_t;
    pub fn sgx_qcnl_free_qe_identity(p_qe_identity: *const uint8_t);
    pub fn sgx_qcnl_get_qve_identity(pp_qve_identity: *mut *mut c_char,
                                     p_qve_identity_size: *mut uint32_t,
                                     pp_qve_identity_issuer_chain: *mut *mut c_char,
                                     p_qve_identity_issuer_chain_size: *mut uint32_t) -> sgx_qcnl_error_t;
    pub fn sgx_qcnl_free_qve_identity(p_qve_identity: *const c_char, p_qve_identity_issuer_chain: *const c_char);
    pub fn sgx_qcnl_get_root_ca_crl(p_root_ca_crl: *mut *mut uint8_t, p_root_ca_cal_size: *mut uint16_t) -> sgx_qcnl_error_t;
    pub fn sgx_qcnl_free_root_ca_crl(p_root_ca_crl: *const uint8_t);
    pub fn sgx_qcnl_register_platform(p_pck_cert_id: *const sgx_ql_pck_cert_id_t,
                                      platform_manifest: *const uint8_t,
                                      platform_manifest_size: uint16_t,
                                      user_token: *const uint8_t,
                                      user_token_size: uint16_t) -> sgx_qcnl_error_t;
    pub fn sgx_qcnl_get_api_version() -> int32_t;
}

//#[link(name = "dcap_quoteverify")]
extern "C" {
    //
    // sgx_dcap_quoteverify.h
    //
    pub fn sgx_qv_verify_quote(p_quote: *const uint8_t,
                               quote_size: uint32_t,
                               p_quote_collateral: *const sgx_ql_qve_collateral_t,
                               expiration_check_date: time_t,
                               p_collateral_expiration_status: *mut uint32_t,
                               p_quote_verification_result: *mut sgx_ql_qv_result_t,
                               p_qve_report_info: *mut sgx_ql_qe_report_info_t,
                               supplemental_data_size: uint32_t,
                               p_supplemental_data: *mut uint8_t) -> sgx_quote3_error_t;
    pub fn sgx_qv_get_quote_supplemental_data_size(p_data_size: *mut uint32_t) -> sgx_quote3_error_t;
    pub fn sgx_qv_set_enclave_load_policy(policy: sgx_ql_request_policy_t) -> sgx_quote3_error_t;

    /* intel DCAP 1.5 */
    pub fn sgx_qv_get_qve_identity(pp_qveid: *mut *mut uint8_t,
                                   p_qveid_size: *mut uint32_t,
                                   pp_qveid_issue_chain: *mut *mut uint8_t,
                                   p_qveid_issue_chain_size: *mut uint32_t,
                                   pp_root_ca_crl: *mut *mut uint8_t,
                                   p_root_ca_crl_size: *mut uint16_t) -> sgx_quote3_error_t;

    pub fn sgx_qv_free_qve_identity(p_qveid: *const uint8_t,
                                    p_qveid_issue_chain: *const uint8_t,
                                    p_root_ca_crl: *const uint8_t) -> sgx_quote3_error_t;

    /* intel DCAP 1.6 */
    pub fn sgx_qv_set_path(path_type: sgx_qv_path_type_t, p_path: *const c_char) -> sgx_quote3_error_t;
}

/* intel DCAP 1.7 */
//#[link(name = "sgx_dcap_tvl")]
extern "C" {
    //
    // sgx_dcap_tvl.h
    //
    pub fn sgx_tvl_verify_qve_report_and_identity(p_quote: *const uint8_t,
                                                  quote_size: uint32_t,
                                                  p_qve_report_info: *const sgx_ql_qe_report_info_t,
                                                  expiration_check_date: time_t,
                                                  collateral_expiration_status: uint32_t,
                                                  quote_verification_result: sgx_ql_qv_result_t,
                                                  p_supplemental_data: *const uint8_t,
                                                  supplemental_data_size: uint32_t,
                                                  qve_isvsvn_threshold: sgx_isv_svn_t) -> sgx_quote3_error_t;
}
