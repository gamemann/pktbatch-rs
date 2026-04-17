use anyhow::Result;

use async_trait::async_trait;

use crate::context::Context;

#[async_trait]
pub trait TechExt {
    type Tech;
    type Opts;
    type TechData;

    /// Creates a new instance of the packet tech with the given options.
    ///
    /// # Arguments
    /// * `opts` - The options for configuring the packet tech.
    ///
    /// # Returns
    /// * `Self` - A new instance of the packet tech.
    fn new(opts: Self::Opts) -> Self;

    /// Retrieves a reference to the underlying packet tech.
    ///
    /// # Returns
    /// * `&Self::Tech` - A reference to the underlying packet tech.
    fn get(&self) -> &Self::Tech;

    /// Retrieves a mutable reference to the underlying packet tech.
    ///
    /// # Returns
    /// * `&mut Self::Tech` - A mutable reference to the underlying packet tech.
    fn get_mut(&mut self) -> &mut Self::Tech;

    /// Initializes the packet tech. This is where setup takes place.
    ///
    /// # Arguments
    /// * `ctx` - The context of the application, which contains shared data and resources.
    ///
    /// # Returns
    /// * `Result<()>` - Returns `Ok(())` if initialization is successful, or an error if it fails.
    async fn init(&mut self, ctx: Context) -> Result<()>;

    /// Sends a packet.
    ///
    /// # Arguments
    /// * `ctx` - The context of the application, which contains shared data and resources.
    /// * `pkt` - The packet data to be sent.
    /// * `data` - Additional data specific to the packet tech, which may be required for sending the packet.
    ///
    /// # Returns
    /// * `Result<()>` - Returns `Ok(())` if the packet is sent successfully, or an error if it fails.
    fn pkt_send(&mut self, ctx: Context, pkt: &[u8], data: Self::TechData) -> Result<()>;
}
