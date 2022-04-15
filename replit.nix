{ pkgs }: {
    deps = [
				# rust deps
        pkgs.rustc
				pkgs.cargo
				pkgs.pkg-config
				pkgs.openssl
			
				# Js deps
				pkgs.nodejs
    ];
		PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}