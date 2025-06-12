pub struct PidfinderOpts {
    pub pid: u32,
    pub method: Method,
}

enum Method {
    Stationary1,
    Stationaryj,
    Stationaryk,
    Wildj,
    Wildk,
}

fn find_pid(opts: &PidfinderOpts) -> Option<Gen4StaticPokemon> {
    let targetpid = opts.pid;
    let pidl = (targetpid & 0xFFFF) as u16;
    let pidh = (targetpid >> 16) as u16;
}
