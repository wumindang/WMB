// 文件路径: WMB/core/src/ffi.rs

use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use crate::crypto::Crypto;
use secp256k1::{SecretKey, PublicKey};
use crate::blockchain::Transaction;
use crate::network::P2pNetwork;

#[no_mangle]
pub extern "C" fn generate_keypair(private_key: *mut *mut c_char, public_key: *mut *mut c_char) {
    let crypto = Crypto::new();
    let (secret_key, public_key_obj) = crypto.generate_keypair();

    let private_key_hex = hex::encode(secret_key.secret_bytes());
    let public_key_hex = hex::encode(public_key_obj.serialize());

    let private_key_cstr = CString::new(private_key_hex).unwrap();
    let public_key_cstr = CString::new(public_key_hex).unwrap();

    unsafe {
        *private_key = private_key_cstr.into_raw();
        *public_key = public_key_cstr.into_raw();
    }
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    unsafe {
        if !ptr.is_null() {
            let _ = CString::from_raw(ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn sign_transaction(sender: *const c_char, receiver: *const c_char, amount: f64, fee: f64, private_key: *const c_char, signature: *mut *mut c_char) {
    let crypto = Crypto::new();

    let sender = unsafe { CStr::from_ptr(sender).to_string_lossy().into_owned() };
    let receiver = unsafe { CStr::from_ptr(receiver).to_string_lossy().into_owned() };
    let private_key_hex = unsafe { CStr::from_ptr(private_key).to_string_lossy().into_owned() };

    let private_key_bytes = hex::decode(private_key_hex).unwrap();
    let secret_key = SecretKey::from_slice(&private_key_bytes).unwrap();

    let mut tx = Transaction {
        sender,
        receiver,
        amount,
        fee,
        signature: Vec::new(),
    };

    crypto.sign_transaction(&mut tx, &secret_key);

    let signature_hex = hex::encode(&tx.signature);
    let signature_cstr = CString::new(signature_hex).unwrap();

    unsafe {
        *signature = signature_cstr.into_raw();
    }
}

#[no_mangle]
pub extern "C" fn broadcast_transaction(tx_sender: *const c_char, tx_receiver: *const c_char, tx_amount: f64, tx_fee: f64, tx_signature: *const c_char) {
    let sender = unsafe { CStr::from_ptr(tx_sender).to_string_lossy().into_owned() };
    let receiver = unsafe { CStr::from_ptr(tx_receiver).to_string_lossy().into_owned() };
    let signature_hex = unsafe { CStr::from_ptr(tx_signature).to_string_lossy().into_owned() };
    let signature = hex::decode(signature_hex).unwrap();

    let tx = Transaction {
        sender,
        receiver,
        amount: tx_amount,
        fee: tx_fee,
        signature,
    };

    let keypair = libp2p::identity::Keypair::generate_ed25519();
    let mut network = P2pNetwork::new(keypair).unwrap();
    network.listen("/ip4/0.0.0.0/tcp/0").unwrap();

    network.broadcast_transaction(tx).unwrap();
}