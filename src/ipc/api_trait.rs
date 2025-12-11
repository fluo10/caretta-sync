use std::time::Duration;

use crate::{entity::{authorized_device, invitation_token}, ipc::{DevicePingRequest, DevicePingResponse}, types::{DeviceIdentifier, InvitationToken}};

/// A trait for 
#[async_trait::async_trait]
pub trait IpcApiTrait{
    type Error;
    /// Get device information
    async fn device_get(&self, target: DeviceIdentifier) -> Result<authorized_device::Model, Self::Error>;
    
    /// List device information
    async fn device_list(&self) -> Result<Vec<authorized_device::Model>, Self::Error>;
    
    /// Ping device.
    /// 
    /// This function is for connectivity test so it's works between non-authorized devices.
    async fn device_ping(&self, target: DeviceIdentifier) -> Result<Duration, Self::Error>;
    
    /// Remove target device from authorized device table.
    async fn device_remove(&self, target: DeviceIdentifier) -> Result<(), Self::Error>;

    /// Get token information
    async fn token_get(&self, id: u32) -> Result<invitation_token::Model, Self::Error>;

    /// List tokens
    async fn token_list(&self) -> Result<Vec<invitation_token::Model>, Self::Error>;

    /// Revoke target token.
    async fn token_revoke(&self, id: u32) -> Result<(), Self::Error>;

    /// Initialize empty user data.
    async fn init(&self) -> Result<(), Self::Error>;
    
    /// Create new token
    async fn invite(&self) -> Result<InvitationToken, Self::Error>;

    /// Join existing cluster and import its user data
    async fn join(&self, token: InvitationToken) -> Result<(), Self::Error>;
}