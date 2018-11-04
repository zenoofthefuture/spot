use {docopt, io, net};

error_chain! {
	foreign_links {
		Docopt(docopt::Error);
		Io(io::Error);
		Network(net::Error);
		SockAddr(std::net::AddrParseError);
	}
}
