with import <nixpkgs> {};
stdenv.mkDerivation rec {
    # LIBCLANG_PATH = "${llvmPackages.libclang}/lib"; 

    RUSTC_WRAPPER= "${sccache}/bin/sccache";
    FONTCONFIG_FILE = "${pkgs.fontconfig.out}/etc/fonts/fonts.conf";
    # LD_LIBRARY_PATH = "${pkgs.xorg.libXcursor}/lib:${pkgs.xorg.libXrandr}/lib:${pkgs.xorg.libX11}/lib:${pkgs.xorg.libXi}/lib:${pkgs.libGL}/lib";
    name = "Shell";
    src = null;
    nativeBuildInputs = [ wrapGAppsHook ];
    buildInputs = [ 
        # gtk
        graphene 
        pango
        cairo
        gdk-pixbuf
        gtk4
        glade

        cmake gcc
        rustup openssl 
        pkgconfig libudev clang 
        nodejs  sccache steam-run 
        glib 

        fontconfig
        freetype
        expat

        # opengl + LD_LIBRARY_PATH
        xorg.libXcursor
        xorg.libX11
        xorg.libXrandr
        xorg.libXi
        libGL
        libxkbcommon
        vulkan-tools
        lutris

        SDL2 SDL2_gfx SDL2_image

        # gfortran
        openblas
        gdb
        python3

        postgresql
    ];
}
