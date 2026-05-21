######################################
#
# dm-polypitch
#
######################################

# DM_POLYPITCH_VERSION = <SHA>
# DM_POLYPITCH_SITE = https://github.com/davemollen/dm-PolyPitch.git
# DM_POLYPITCH_SITE_METHOD = git
DM_POLYPITCH_VERSION = 1
DM_POLYPITCH_SITE = /root/source
DM_POLYPITCH_SITE_METHOD = local
DM_POLYPITCH_BUNDLES = dm-PolyPitch.lv2

define DM_POLYPITCH_BUILD_CMDS
	~/.cargo/bin/rustup default nightly

	rm -f $(@D)/lv2/dm-PolyPitch.lv2/libdm_poly_pitch.so
	(cd $(@D)/lv2 && \
		~/.cargo/bin/cargo build $(MOD_PLUGIN_BUILDER_RUST_BUILD_FLAGS))

	~/.cargo/bin/rustup default stable
endef

define DM_POLYPITCH_INSTALL_TARGET_CMDS
	$(INSTALL) -d $(TARGET_DIR)/usr/lib/lv2
	cp -rv $(@D)/lv2/dm-PolyPitch.lv2 $(TARGET_DIR)/usr/lib/lv2/
	$(INSTALL) -m 644 $(@D)/lv2/target/$(MOD_PLUGIN_BUILDER_RUST_TARGET)/release/libdm_poly_pitch.so $(TARGET_DIR)/usr/lib/lv2/dm-PolyPitch.lv2/
endef

$(eval $(generic-package))
