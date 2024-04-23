pub extern fn strcmp(__s1: [*c]const u8, __s2: [*c]const u8) c_int;
pub const __off_t = c_long;
pub extern fn mmap(__addr: ?*anyopaque, __len: usize, __prot: c_int, __flags: c_int, __fd: c_int, __offset: __off_t) ?*anyopaque;
pub extern fn memcpy(__dest: ?*anyopaque, __src: ?*const anyopaque, __n: c_ulong) ?*anyopaque;

const std = @import("std");
const drm = @cImport({
    @cInclude("xf86drmMode.h");
    @cInclude("xf86drm.h");
});
const print = std.debug.print;

pub export fn get_property_value(arg_drm_fd: c_int, arg_object_id: u32, arg_object_type: u32, arg_prop_name: [*c]const u8) u64 {
    var drm_fd = arg_drm_fd;
    var object_id = arg_object_id;
    var object_type = arg_object_type;
    var prop_name = arg_prop_name;
    var props: [*c]drm.drmModeObjectProperties = drm.drmModeObjectGetProperties(drm_fd, object_id, object_type);
    {
        var i: u32 = 0;
        while (i < props.*.count_props) : (i +%= 1) {
            var prop: [*c]drm.drmModePropertyRes = drm.drmModeGetProperty(drm_fd, props.*.props[i]);
            var val: u64 = props.*.prop_values[i];
            if (strcmp(@as([*c]u8, @ptrCast(@alignCast(&prop.*.name))), prop_name) == @as(c_int, 0)) {
                drm.drmModeFreeProperty(prop);
                drm.drmModeFreeObjectProperties(props);
                return val;
            }
            drm.drmModeFreeProperty(prop);
        }
    }
    std.os.abort();
}

pub export fn add_property(arg_drm_fd: c_int, arg_req: ?*drm.drmModeAtomicReq, arg_object_id: u32, arg_object_type: u32, arg_prop_name: [*c]const u8, arg_value: u64) void {
    var drm_fd = arg_drm_fd;
    var req = arg_req;
    var object_id = arg_object_id;
    var object_type = arg_object_type;
    var prop_name = arg_prop_name;
    var value = arg_value;
    var prop_id: u32 = 0;
    var props: [*c]drm.drmModeObjectProperties = drm.drmModeObjectGetProperties(drm_fd, object_id, object_type);
    {
        var i: u32 = 0;
        while (i < props.*.count_props) : (i +%= 1) {
            var prop: [*c]drm.drmModePropertyRes = drm.drmModeGetProperty(drm_fd, props.*.props[i]);
            if (strcmp(@as([*c]u8, @ptrCast(@alignCast(&prop.*.name))), prop_name) == @as(c_int, 0)) {
                prop_id = prop.*.prop_id;
                break;
            }
        }
    }
    _ = blk: {
        _ = @sizeOf(c_int);
        break :blk blk_1: {
            break :blk_1 if (prop_id != @as(u32, @bitCast(@as(c_int, 0)))) {} else {
                // __assert_fail("prop_id != 0", "04-animatefb.c", @as(c_uint, @bitCast(@as(c_int, 52))), "void add_property(int, drmModeAtomicReq *, uint32_t, uint32_t, const char *, uint64_t)");
                std.os.abort();
            };
        };
    };
    _ = drm.drmModeAtomicAddProperty(req, object_id, prop_id, value);
}

pub export var crtc: [*c]drm.drmModeCrtc = null;
// pub export var mode: [*c]drm.drmModeModeInfo = null;
pub export var mode: drm.drmModeModeInfo = undefined;
pub export var plane: [*c]drm.drmModePlane = null;

pub fn main() u8 {
    var drm_fd: c_int = std.c.open("/dev/dri/card0", @as(c_int, 2) | @as(c_int, 2048));
    if (drm_fd < @as(c_int, 0)) {
        // perror("open failed");
        return 1;
    }
    if (drm.drmSetClientCap(drm_fd, @as(u64, @bitCast(@as(c_long, @as(c_int, 2)))), @as(u64, @bitCast(@as(c_long, @as(c_int, 1))))) != @as(c_int, 0)) {
        // perror("drmSetClientCap(UNIVERSAL_PLANES) failed");
        return 1;
    }
    if (drm.drmSetClientCap(drm_fd, @as(u64, @bitCast(@as(c_long, @as(c_int, 3)))), @as(u64, @bitCast(@as(c_long, @as(c_int, 1))))) != @as(c_int, 0)) {
        // perror("drmSetClientCap(ATOMIC) failed");
        return 1;
    }
    var resources: [*c]drm.drmModeRes = drm.drmModeGetResources(drm_fd);
    {
        var i: c_int = 0;
        while (i < resources.*.count_crtcs) : (i += 1) {
            var crtc_id: u32 = (blk: {
                const tmp = i;
                if (tmp >= 0) break :blk resources.*.crtcs + @as(usize, @intCast(tmp)) else break :blk resources.*.crtcs - ~@as(usize, @bitCast(@as(isize, @intCast(tmp)) +% -1));
            }).*;
            crtc = drm.drmModeGetCrtc(drm_fd, crtc_id);
            if (crtc.*.mode_valid != 0) {
                break;
            }
            drm.drmModeFreeCrtc(crtc);
            crtc = null;
        }
    }
    _ = blk: {
        _ = @sizeOf(c_int);
        break :blk blk_1: {
            break :blk_1 if (crtc != @as([*c]drm.drmModeCrtc, @ptrCast(@alignCast(@as(?*anyopaque, @ptrFromInt(@as(c_int, 0))))))) {} else {
                // __assert_fail("crtc != NULL", "04-animatefb.c", @as(c_uint, @bitCast(@as(c_int, 85))), "int main(int, char **)");
                std.c.abort();
            };
        };
    };
    // _ = printf("Using CRTC %u\n", crtc.*.crtc_id);
    mode = crtc.*.mode;
    // _ = printf("Using mode %dx%d %dHz\n", @as(c_int, @bitCast(@as(c_uint, mode.hdisplay))), @as(c_int, @bitCast(@as(c_uint, mode.vdisplay))), mode.vrefresh);
    var planes: [*c]drm.drmModePlaneRes = drm.drmModeGetPlaneResources(drm_fd);
    {
        var i: u32 = 0;
        while (i < planes.*.count_planes) : (i +%= 1) {
            var plane_id: u32 = planes.*.planes[i];
            plane = drm.drmModeGetPlane(drm_fd, plane_id);
            var plane_type: u64 = get_property_value(drm_fd, plane_id, @as(c_uint, 4008636142), "type");
            if ((plane.*.crtc_id == crtc.*.crtc_id) and (plane_type == @as(u64, @bitCast(@as(c_long, @as(c_int, 1)))))) {
                break;
            }
            drm.drmModeFreePlane(plane);
            plane = null;
        }
    }
    _ = blk: {
        _ = @sizeOf(c_int);
        break :blk blk_1: {
            break :blk_1 if (plane != @as([*c]drm.drmModePlane, @ptrCast(@alignCast(@as(?*anyopaque, @ptrFromInt(@as(c_int, 0))))))) {} else {
                std.c.abort();
                // __assert_fail("plane != NULL", "04-animatefb.c", @as(c_uint, @bitCast(@as(c_int, 105))), "int main(int, char **)");
            };
        };
    };
    // _ = printf("Using plane %u\n", plane.*.plane_id);
    drm.drmModeFreePlaneResources(planes);
    drm.drmModeFreeResources(resources);
    var width: c_int = @as(c_int, @bitCast(@as(c_uint, mode.hdisplay)));
    var height: c_int = @as(c_int, @bitCast(@as(c_uint, mode.vdisplay)));

    var create: drm.drm_mode_create_dumb = .{
        .height = @as(u32, @bitCast(height)),
        .width = @as(u32, @bitCast(width)),
        .bpp = @as(u32, @bitCast(@as(c_int, 32))),
        .flags = @import("std").mem.zeroes(u32),
        .handle = @import("std").mem.zeroes(u32),
        .pitch = @import("std").mem.zeroes(u32),
        .size = @import("std").mem.zeroes(u64),
    };
    _ = drm.drmIoctl(drm_fd, @as(c_ulong, @bitCast(@as(c_ulong, (((@as(c_uint, 2) | @as(c_uint, 1)) << @intCast(((@as(c_int, 0) + @as(c_int, 8)) + @as(c_int, 8)) + @as(c_int, 14))) | @as(c_uint, @bitCast(@as(c_int, 'd') << @intCast(@as(c_int, 0) + @as(c_int, 8))))) | @as(c_uint, @bitCast(@as(c_int, 178) << @intCast(0)))))) | (@sizeOf(drm.drm_mode_create_dumb) << @intCast((@as(c_int, 0) + @as(c_int, 8)) + @as(c_int, 8))), @as(?*anyopaque, @ptrCast(&create)));
    var handle: u32 = create.handle;
    var stride: u32 = create.pitch;
    var size: u32 = @as(u32, @bitCast(@as(c_uint, @truncate(create.size))));
    var handles: [4]u32 = [1]u32{
        handle,
    } ++ [1]u32{@import("std").mem.zeroes(u32)} ** 3;
    var strides: [4]u32 = [1]u32{
        stride,
    } ++ [1]u32{@import("std").mem.zeroes(u32)} ** 3;
    var offsets: [4]u32 = [1]u32{
        0,
    } ++ [1]u32{@import("std").mem.zeroes(u32)} ** 3;
    var fb_id: u32 = 0;
    _ = drm.drmModeAddFB2(drm_fd, @as(u32, @bitCast(width)), @as(u32, @bitCast(height)), ((@as(u32, @bitCast(@as(c_int, 'X'))) | (@as(u32, @bitCast(@as(c_int, 'R'))) << @intCast(8))) | (@as(u32, @bitCast(@as(c_int, '2'))) << @intCast(16))) | (@as(u32, @bitCast(@as(c_int, '4'))) << @intCast(24)), @as([*c]u32, @ptrCast(@alignCast(&handles))), @as([*c]u32, @ptrCast(@alignCast(&strides))), @as([*c]u32, @ptrCast(@alignCast(&offsets))), &fb_id, @as(u32, @bitCast(@as(c_int, 0))));
    // _ = printf("Allocated FB %u\n", fb_id);
    var map: drm.drm_mode_map_dumb = .{
        .handle = handle,
        .pad = @import("std").mem.zeroes(u32),
        .offset = @import("std").mem.zeroes(u64),
    };
    _ = drm.drmIoctl(drm_fd, @as(c_ulong, @bitCast(@as(c_ulong, (((@as(c_uint, 2) | @as(c_uint, 1)) << @intCast(((@as(c_int, 0) + @as(c_int, 8)) + @as(c_int, 8)) + @as(c_int, 14))) | @as(c_uint, @bitCast(@as(c_int, 'd') << @intCast(@as(c_int, 0) + @as(c_int, 8))))) | @as(c_uint, @bitCast(@as(c_int, 179) << @intCast(0)))))) | (@sizeOf(drm.drm_mode_map_dumb) << @intCast((@as(c_int, 0) + @as(c_int, 8)) + @as(c_int, 8))), @as(?*anyopaque, @ptrCast(&map)));
    var data: [*c]u8 = @as([*c]u8, @ptrCast(@alignCast(mmap(null, @as(usize, @bitCast(@as(c_ulong, size))), @as(c_int, 1) | @as(c_int, 2), @as(c_int, 1), drm_fd, @as(__off_t, @bitCast(@as(c_ulong, @truncate(map.offset))))))));
    var color: [4]u8 = [4]u8{
        0,
        0,
        255,
        255,
    };
    var inc: c_int = 1;
    var dec: c_int = 2;
    {
        var i: c_int = 0;
        while (i < (@as(c_int, 60) * @as(c_int, 5))) : (i += 1) {
            color[@as(c_uint, @intCast(inc))] +%= @as(u8, @bitCast(@as(i8, @truncate(@as(c_int, 15)))));
            color[@as(c_uint, @intCast(dec))] -%= @as(u8, @bitCast(@as(i8, @truncate(@as(c_int, 15)))));
            if (@as(c_int, @bitCast(@as(c_uint, color[@as(c_uint, @intCast(dec))]))) == @as(c_int, 0)) {
                dec = inc;
                inc = @import("std").zig.c_translation.signedRemainder(inc + @as(c_int, 2), @as(c_int, 3));
            }
            {
                var y: c_int = 0;
                while (y < height) : (y += 1) {
                    {
                        var x: c_int = 0;
                        while (x < width) : (x += 1) {
                            var offset: usize = @as(c_ulong, @bitCast(@as(c_ulong, @as(u32, @bitCast(y)) *% stride))) +% (@as(c_ulong, @bitCast(@as(c_long, x))) *% @sizeOf([4]u8));
                            _ = memcpy(@as(?*anyopaque, @ptrCast(&data[offset])), @as(?*const anyopaque, @ptrCast(@as([*c]u8, @ptrCast(@alignCast(&color))))), @sizeOf([4]u8));
                        }
                    }
                }
            }
            var req: ?*drm.drmModeAtomicReq = drm.drmModeAtomicAlloc();
            var plane_id: u32 = plane.*.plane_id;
            add_property(drm_fd, req, plane_id, @as(c_uint, 4008636142), "FB_ID", @as(u64, @bitCast(@as(c_ulong, fb_id))));
            add_property(drm_fd, req, plane_id, @as(c_uint, 4008636142), "SRC_X", @as(u64, @bitCast(@as(c_long, @as(c_int, 0)))));
            add_property(drm_fd, req, plane_id, @as(c_uint, 4008636142), "SRC_Y", @as(u64, @bitCast(@as(c_long, @as(c_int, 0)))));
            add_property(drm_fd, req, plane_id, @as(c_uint, 4008636142), "SRC_W", @as(u64, @bitCast(@as(c_long, width << @intCast(16)))));
            add_property(drm_fd, req, plane_id, @as(c_uint, 4008636142), "SRC_H", @as(u64, @bitCast(@as(c_long, height << @intCast(16)))));
            add_property(drm_fd, req, plane_id, @as(c_uint, 4008636142), "CRTC_X", @as(u64, @bitCast(@as(c_long, @as(c_int, 0)))));
            add_property(drm_fd, req, plane_id, @as(c_uint, 4008636142), "CRTC_Y", @as(u64, @bitCast(@as(c_long, @as(c_int, 0)))));
            add_property(drm_fd, req, plane_id, @as(c_uint, 4008636142), "CRTC_W", @as(u64, @bitCast(@as(c_long, width))));
            add_property(drm_fd, req, plane_id, @as(c_uint, 4008636142), "CRTC_H", @as(u64, @bitCast(@as(c_long, height))));
            var flags: u32 = @as(u32, @bitCast(@as(c_int, 512)));
            var ret: c_int = drm.drmModeAtomicCommit(drm_fd, req, flags, @as(?*anyopaque, @ptrFromInt(@as(c_int, 0))));
            if (ret != @as(c_int, 0)) {
                // perror("drmModeAtomicCommit failed");
                return 1;
            }

            std.time.sleep(16666667);
        }
    }
    return 0;
}
