use priv_prelude::*;

/// A node representing an Ipv4 endpoint.
pub struct EndpointV4Node<F> {
    func: F,
}

/// Create a node for an Ipv4 endpoint. This node will run the given function in a network
/// namespace with a single interface.
pub fn endpoint_v4<R, F>(func: F) -> EndpointV4Node<F>
where
    R: Send + 'static,
    F: FnOnce(Ipv4Addr) -> R + Send + 'static,
{
    EndpointV4Node { func }
}

impl<R, F> Ipv4Node for EndpointV4Node<F>
where
    R: Send + 'static,
    F: FnOnce(Ipv4Addr) -> R + Send + 'static,
{
    type Output = R;

    fn build(
        self,
        handle: &Handle,
        subnet: SubnetV4,
    ) -> (SpawnComplete<R>, Ipv4Plug) {
        let address = subnet.random_client_addr();
        let iface = {
            Ipv4IfaceBuilder::new()
            .address(address)
            .netmask(subnet.netmask())
            .route(RouteV4::new(SubnetV4::global(), None))
        };
        let (plug_a, plug_b) = Ipv4Plug::new_wire();

        let spawn_complete = {
            MachineBuilder::new()
            .add_ipv4_iface(iface, plug_b)
            .spawn(handle, move || (self.func)(address))
        };

        (spawn_complete, plug_a)
    }
}

