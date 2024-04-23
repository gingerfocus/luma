const std = @import("std");
const drm = @cImport({
    @cInclude("xf86drmMode.h");
    @cInclude("xf86drm.h");
});
const print = std.debug.print;

pub fn main() !void {
    // var drm_fd: c_int = open("/dev/dri/card0", @as(c_int, 2) | @as(c_int, 2048));
    const drm_fd: c_int = try std.os.open("/dev/dri/card0", std.os.O.RDWR | std.os.O.NONBLOCK, 0);
    defer std.os.close(drm_fd);

    if (drm.drmSetClientCap(drm_fd, drm.DRM_CLIENT_CAP_UNIVERSAL_PLANES, 1) != 0) {
        return error.noplane;
    }

    var resources: *drm.drmModeRes = drm.drmModeGetResources(drm_fd);
    defer drm.drmModeFreeResources(resources);

    var i: c_int = 0;
    while (i < resources.*.count_connectors) : (i += 1) {
        var conn_id: u32 = (blk: {
            const tmp = i;
            if (tmp >= 0) break :blk resources.*.connectors + @as(usize, @intCast(tmp)) else break :blk resources.*.connectors - ~@as(usize, @bitCast(@as(isize, @intCast(tmp)) +% -1));
        }).*;

        var conn: *drm.drmModeConnector = drm.drmModeGetConnector(drm_fd, conn_id);
        if (conn.*.connection != @as(c_uint, @bitCast(drm.DRM_MODE_CONNECTED))) {
            drm.drmModeFreeConnector(conn);
            continue;
        }

        _ = print("Modes for connector {d}:\n", .{conn.*.connector_id});

        {
            var i_1: c_int = 0;
            while (i_1 < conn.*.count_modes) : (i_1 += 1) {
                var mode: [*c]drm.drmModeModeInfo = &(blk: {
                    const tmp = i_1;
                    if (tmp >= 0) break :blk conn.*.modes + @as(usize, @intCast(tmp)) else break :blk conn.*.modes - ~@as(usize, @bitCast(@as(isize, @intCast(tmp)) +% -1));
                }).*;
                _ = print("{d}x{d} {d}Hz\n", .{ @as(c_int, @bitCast(@as(c_uint, mode.*.hdisplay))), @as(c_int, @bitCast(@as(c_uint, mode.*.vdisplay))), mode.*.vrefresh });
            }
        }

        drm.drmModeFreeConnector(conn);
    }
}
