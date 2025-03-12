//! # Configuration Module for Monobean
//!
//! This module defines constants and utilities for application configuration.
//!
//! ## Constants Categories
//!
//! - **Application metadata**: Version and online resources
//! - **Runtime configurations**: Application identifiers and file paths
//!
//! ## Utilities
//!
//! Provides helper macros for retrieving settings from configuration sources.
//!

use std::path::PathBuf;

use adw::gio::Settings;
use gtk::prelude::*;

use crate::{core::CoreConfigChanged};

/* Application metadata */
pub const VERSION: &str = "0.0.1";
pub const WEBSITE: &str = "https://github.com/web3infra-foundation/mega";

/* Runtime configurations */
pub const APP_ID: &str = "org.Web3Infrastructure.Monobean";
pub const APP_NAME: &str = "Monobean";
pub const PREFIX: &str = "/org/Web3Infrastructure/Monobean";
pub const MEGA_CONFIG_PATH: &str = "/org/Web3Infrastructure/Monobean/mega/config.toml";
pub const MEGA_HTTPS_KEY: &str = "/org/Web3Infrastructure/Monobean/mega/key.pem";
pub const MEGA_HTTPS_CERT: &str = "/org/Web3Infrastructure/Monobean/mega/cert.pem";

/* Helper functions for mega configs */

/// A macro that retrieves a value from GSettings with type checking and conversion.
///
/// # Arguments
///
/// * `$settings` - A GSettings object reference
/// * `$key` - The settings key to retrieve
/// * `$type` - The Rust type to convert the value to
///
/// # Returns
///
/// The value from GSettings converted to the specified Rust type.
///
/// # Panics
///
/// This macro will panic if:
/// - The setting doesn't exist in the schema
/// - The value can't be converted to the specified type
///
/// # Examples
///
/// ```
/// let settings = gio::Settings::new(APP_ID);
/// let http_port: u32 = get_setting!(settings, "print-std", u32);
/// let log_level: String = get_setting!(settings, "log-level", String);
/// let print_std: bool = get_setting!(settings, "print-std", bool);
/// ```
#[macro_export]
macro_rules! get_setting {
    ($settings:expr, $key:expr, $type:ty) => {
        match std::any::type_name::<$type>() {
            "u32" => $settings.uint($key).to_value().get::<$type>().unwrap(),
            "u64" => $settings.uint64($key).to_value().get::<$type>().unwrap(),
            "i32" => $settings.int($key).to_value().get::<$type>().unwrap(),
            "i64" => $settings.int64($key).to_value().get::<$type>().unwrap(),
            "bool" => $settings.boolean($key).to_value().get::<$type>().unwrap(),
            "alloc::string::String" => $settings.string($key).to_value().get::<$type>().unwrap(),
            _ => $settings.value($key).get::<$type>().unwrap(),
        }
    };
}

/// TODO: So ugly...
/// We should update build.rs and use proc macros to generate this code.
pub fn config_update(setting: &Settings) -> Vec<CoreConfigChanged> {
    let mut update = Vec::new();
    // First, let's extract all settings and compare with defaults

    // Base settings
    let base_dir: String = get_setting!(setting, "base-dir", String);
    if base_dir != "/tmp/.mono" {
        update.push(CoreConfigChanged::BaseDir(base_dir.parse::<PathBuf>().unwrap()));
    }

    // Log settings
    let log_path: String = get_setting!(setting, "log-path", String);
    if !log_path.is_empty() {
        update.push(CoreConfigChanged::LogPath(log_path.parse::<PathBuf>().unwrap()));
    }

    let log_level: String = get_setting!(setting, "log-level", String);
    if log_level != "info" {
        update.push(CoreConfigChanged::Level(log_level));
    }

    let print_std: bool = get_setting!(setting, "print-std", bool);
    if !print_std {  // Default is true
        update.push(CoreConfigChanged::PrintStd(print_std));
    }

    // Database settings
    let db_type: String = get_setting!(setting, "db-type", String);
    if db_type != "sqlite" {
        update.push(CoreConfigChanged::DbType(db_type));
    }

    let db_path: String = get_setting!(setting, "db-path", String);
    if !db_path.is_empty() {
        update.push(CoreConfigChanged::DbPath(db_path));
    }

    let db_url: String = get_setting!(setting, "db-url", String);
    if db_url != "postgres://mono:mono@localhost:5432/mono" {
        update.push(CoreConfigChanged::DbUrl(db_url));
    }

    let max_connections: u32 = get_setting!(setting, "max-connections", u32);
    if max_connections != 16 {
        update.push(CoreConfigChanged::MaxConnection(max_connections));
    }

    let min_connections: u32 = get_setting!(setting, "min-connections", u32);
    if min_connections != 8 {
        update.push(CoreConfigChanged::MinConnection(min_connections));
    }

    let sqlx_logging: bool = get_setting!(setting, "sqlx-logging", bool);
    if sqlx_logging {  // Default is false
        update.push(CoreConfigChanged::SqlxLogging(sqlx_logging));
    }

    // Storage settings
    let obs_access_key: String = get_setting!(setting, "obs-access-key", String);
    if !obs_access_key.is_empty() {
        update.push(CoreConfigChanged::ObsAccessKey(obs_access_key));
    }

    let obs_secret_key: String = get_setting!(setting, "obs-secret-key", String);
    if !obs_secret_key.is_empty() {
        update.push(CoreConfigChanged::ObsSecretKey(obs_secret_key));
    }

    let obs_region: String = get_setting!(setting, "obs-region", String);
    if obs_region != "cn-east-3" {
        update.push(CoreConfigChanged::ObsRegion(obs_region));
    }

    let obs_endpoint: String = get_setting!(setting, "obs-endpoint", String);
    if obs_endpoint != "https://obs.cn-east-3.myhuaweicloud.com" {
        update.push(CoreConfigChanged::ObsEndpoint(obs_endpoint));
    }

    // Monorepo settings
    let import_dir: String = get_setting!(setting, "import-dir", String);
    if import_dir != "/third-part" {
        update.push(CoreConfigChanged::ImportDir(import_dir.parse::<PathBuf>().unwrap()));
    }

    let admin: String = get_setting!(setting, "admin", String);
    if admin != "admin" {
        update.push(CoreConfigChanged::Admin(admin));
    }

    let root_dirs: String = get_setting!(setting, "root-dirs", String);
    if root_dirs != "third-part, project, doc, release" {
        // Convert comma-separated string to Vec<String>
        let dirs: Vec<String> = root_dirs.split(',')
            .map(|s| s.trim().to_string())
            .collect();
        update.push(CoreConfigChanged::RootDirs(dirs));
    }

    // Authentication settings
    let http_auth: bool = get_setting!(setting, "http-auth", bool);
    if http_auth {  // Default is false
        update.push(CoreConfigChanged::EnableHttpAuth(http_auth));
    }

    let test_user: bool = get_setting!(setting, "test-user", bool);
    if !test_user {  // Default is true
        update.push(CoreConfigChanged::EnableTestUser(test_user));
    }

    let test_user_name: String = get_setting!(setting, "test-user-name", String);
    if test_user_name != "mega" {
        update.push(CoreConfigChanged::TestUserName(test_user_name));
    }

    let test_user_token: String = get_setting!(setting, "test-user-token", String);
    if test_user_token != "mega" {
        update.push(CoreConfigChanged::TestUserToken(test_user_token));
    }

    // Pack settings
    let pack_decode_mem_size: String = get_setting!(setting, "pack-decode-mem-size", String);
    if pack_decode_mem_size != "4G" {
        update.push(CoreConfigChanged::PackDecodeMemSize(pack_decode_mem_size));
    }

    let pack_decode_disk_size: String = get_setting!(setting, "pack-decode-disk-size", String);
    if pack_decode_disk_size != "20%" {
        update.push(CoreConfigChanged::PackDecodeDiskSize(pack_decode_disk_size));
    }

    let pack_decode_cache_path: String = get_setting!(setting, "pack-decode-cache-path", String);
    if !pack_decode_cache_path.is_empty() {
        update.push(CoreConfigChanged::PackDecodeCachePath(pack_decode_cache_path.parse::<PathBuf>().unwrap()));
    }

    let clean_cache: bool = get_setting!(setting, "clean-cache", bool);
    if !clean_cache {  // Default is true
        update.push(CoreConfigChanged::CleanCacheAfterDecode(clean_cache));
    }

    let channel_message_size: u32 = get_setting!(setting, "channel-message-size", u32);
    if channel_message_size != 1000000 {
        update.push(CoreConfigChanged::ChannelMessageSize(channel_message_size as usize));
    }

    // LFS settings
    let lfs_url: String = get_setting!(setting, "lfs-url", String);
    if lfs_url != "http://localhost:8000" {
        update.push(CoreConfigChanged::LfsUrl(lfs_url));
    }

    let lfs_obj_local_path: String = get_setting!(setting, "lfs-obj-local-path", String);
    if !lfs_obj_local_path.is_empty() {
        update.push(CoreConfigChanged::LfsObjLocalPath(lfs_obj_local_path.parse::<PathBuf>().unwrap()));
    }

    let enable_split: bool = get_setting!(setting, "enable-split", bool);
    if !enable_split {  // Default is true
        update.push(CoreConfigChanged::EnableSplit(enable_split));
    }

    let split_size: String = get_setting!(setting, "split-size", String);
    if split_size != "20M" {
        update.push(CoreConfigChanged::SplitSize(split_size));
    }

    // OAuth settings
    let github_client_id: String = get_setting!(setting, "github-client-id", String);
    if !github_client_id.is_empty() {
        update.push(CoreConfigChanged::GithubClientId(github_client_id));
    }

    let github_client_secret: String = get_setting!(setting, "github-client-secret", String);
    if !github_client_secret.is_empty() {
        update.push(CoreConfigChanged::GithubClientSecret(github_client_secret));
    }

    let ui_domain: String = get_setting!(setting, "ui-domain", String);
    if ui_domain != "http://localhost:3000" {
        update.push(CoreConfigChanged::UiDomain(ui_domain));
    }

    let cookie_domain: String = get_setting!(setting, "cookie-domain", String);
    if cookie_domain != "localhost" {
        update.push(CoreConfigChanged::CookieDomain(cookie_domain));
    }

    update

}
