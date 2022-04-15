{ pkgs }: {
    deps = [
				# rust deps
        pkgs.rustc
				pkgs.cargo
				# Js deps
				pkgs.nodejs
    ];
}