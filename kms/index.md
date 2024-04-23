class: center, middle, inverse

# Getting pixels on screen on Linux

## Introduction to Kernel Mode Setting

Simon Ser (emersion)

SourceHut

???

45min talk, 15min QA

Lacks docs (outdated, geared towards driver devs)

Agenda: TODO

---

class: center, middle, inverse

# 1. What is KMS?

???

---

## How to get an image on screen?

Regular apps don't talk to the kernel for display
--

Regular apps talk to a display server: e.g. X11 or Wayland

<img src="img/compositor.svg" style="width: 80%"/>

???

- The compositor blends images coming from multiple clients
-

---

## How to get an image on screen?

<img src="img/kms.svg" style="width: 50%"/>

???

- KMS offers a vendor-agnostic API
- KMS offers a low-level API

---

## Why use KMS?

Don't!
--

Program with exclusive access to outputs which needs low-level control over the
GPU

- Display servers
- Media players, some embedded use-cases
- VR and XR

???

- Wayland compositors: Mutter, KWin, Sway/wlroots, Weston
- mpv, Kodi
- Games, Monado

---

## Why learn about KMS?

- Understand how it works
- Contribute to existing projects
- Write a new program which needs KMS

---

class: center, middle, inverse

# 2. Getting an image on screen

???

- Display a red screen with KMS
- We don't care about legacy APIs

---

## Opening the GPU

```c
int drm_fd = open("/dev/dri/card0", O_RDWR | O_NONBLOCK);
```

???

- Primary node
- TODO: error handling
- List devices with udev

---

## DRM resources

```c
drmModeRes *resources = drmModeGetResources(drm_fd);
```
--

```c
typedef struct {
	int count_fbs;
	uint32_t *fbs;

	int count_crtcs;
	uint32_t *crtcs;

	int count_connectors;
	uint32_t *connectors;

	int count_encoders;
	uint32_t *encoders;

	uint32_t min_width, max_width;
	uint32_t min_height, max_height;
} drmModeRes;
```

???

Each object has an ID and a type

drm_info demo

---

## Connectors

```c
drmModeConnector *conn = drmModeGetConnector(drm_fd, conn_id);
```
--

```c
typedef struct {
	uint32_t connector_id;
	uint32_t encoder_id; /* Encoder currently connected to */
	uint32_t connector_type;
	uint32_t connector_type_id;
	drmModeConnection connection;
	uint32_t mmWidth, mmHeight; /* HxW in millimeters */
	drmModeSubPixel subpixel;

	int count_modes;
	drmModeModeInfo *modes;

	int count_props;
	uint32_t *props; /* List of property ids */
	uint64_t *prop_values; /* List of property values */

	int count_encoders;
	uint32_t *encoders; /* List of encoder ids */
} drmModeConnector;
```

???

- Physical connector on the back of the GPU
- e.g. DisplayPort, HDMI, DVI, VGA, USB-C…
- Special case: DisplayPort Multi-Stream Transport (# of connectors can change)

---

## Modes

```c
typedef struct {
	uint32_t clock;
	uint16_t hdisplay, hsync_start, hsync_end, htotal, hskew;
	uint16_t vdisplay, vsync_start, vsync_end, vtotal, vscan;

	uint32_t vrefresh;

	uint32_t flags;
	uint32_t type;
	char name[DRM_DISPLAY_MODE_LEN];
} drmModeModeInfo;
```
--

```c
drmModeConnector *conn = drmModeGetConnector(drm_fd, conn_id);
if (conn->connection != DRM_MODE_CONNECTED) {
	return;
}
printf("Modes for connector %u:\n", conn->connector_id);
for (int i = 0; i < conn->count_modes; i++) {
	drmModeModeInfo *mode = &conn->modes[i];
	printf("%dx%d %dHz\n", mode->hdisplay, mode->vdisplay, mode->vrefresh);
}
```

???

- Resolution + refresh rate (+ other things)
- Refresh rate is fixed
- Changing a mode can take seconds (w/ black screen)
- Note: `vrefresh` not very accurate
- Demo: `01-modelist`

---

## Framebuffers

```c
typedef struct {
	uint32_t fb_id;
	uint32_t width, height;
	uint32_t pitch;
	uint32_t bpp;
	uint32_t depth;
	uint32_t handle; /* driver specific handle */
} drmModeFB;
```

???

Slice of memory containing pixels

---

## DRM formats

Describes how pixels are laid out in the buffer

List in `/usr/include/libdrm/drm_fourcc.h`

`XRGB8888` and `ARGB8888` are likely supported

Example: `#112233` (R=`0x11`, G=`0x22`, B=`0x33`, A=`0xFF`)

- `ARGB8888`: `{ 0x33, 0x22, 0x11, 0xFF }`
- `BGRA8888`: `{ 0xFF, 0x11, 0x22, 0x33 }`

More complicated formats: `XRGB2101010`, `YUYV`, `NV12`

???

- Number of bits per component
- Little-endian
- YCbCr: luma, blue- and red-different chroma
- NV12: YCbCr with 2 planes

---

## Allocating a framebuffer

```c
// Allocate a buffer and get a driver-specific handle back
struct drm_mode_create_dumb create = {
	.width = width,
	.height = height,
	.bpp = 32,
};
drmIoctl(drm_fd, DRM_IOCTL_MODE_CREATE_DUMB, &create);
uint32_t handle = create.handle;
uint32_t stride = create.pitch;
uint32_t size = create.size;

// Create the DRM framebuffer object
uint32_t handles[4] = { handle };
uint32_t strides[4] = { stride };
uint32_t offsets[4] = { 0 };
uint32_t fb_id = 0;
drmModeAddFB2(drm_fd, width, height, DRM_FORMAT_XRGB8888,
	handles, strides, offsets, &fb_id, 0);

// Create a memory mapping
struct drm_mode_map_dumb map = { .handle = handle };
drmIoctl(drm_fd, DRM_IOCTL_MODE_MAP_DUMB, &map);
void *data = mmap(0, size, PROT_READ | PROT_WRITE, MAP_SHARED, drm_fd, map.offset);
```

---

## CRTCs and planes

<img src="img/pipeline.svg"/>

???

CRTCs has nothing to do with Cathode Ray Tubes

---

## CRTCs and planes: why do they exist?

2 connectors wired up to a single CRTC: "clone screens"

<img src="img/pipeline-2conn.svg"/>

---

## CRTCs and planes: why do they exist?

2 planes wired up to a single CRTC: show multiple framebuffers

<img src="img/pipeline-2planes.svg" style="width: 70%"/>

---

## CRTCs

```c
drmModeCrtc *crtc = drmModeGetCrtc(drm_fd, crtc_id);
```
--

```c
typedef struct {
	uint32_t crtc_id;
	uint32_t buffer_id; /* FB id to connect to 0 = disconnect */

	uint32_t x, y; /* Position on the framebuffer */
	uint32_t width, height;
	int mode_valid;
	drmModeModeInfo mode;

	int gamma_size; /* Number of gamma stops */
} drmModeCrtc;
```

---

## Planes

```c
drmSetClientCap(drm_fd, DRM_CLIENT_CAP_UNIVERSAL_PLANES, 1);

drmModePlaneRes *planes = drmModeGetPlaneResources(drm_fd);
for (uint32_t i = 0; i < planes->count_planes; i++) {
	uint32_t plane_id = planes->planes[i];
	drmModePlane *plane = drmModeGetPlane(drm_fd, plane_id);
}
```
--

```c
typedef struct {
	uint32_t count_formats;
	uint32_t *formats;
	uint32_t plane_id;

	uint32_t crtc_id;
	uint32_t fb_id;

	uint32_t crtc_x, crtc_y;
	uint32_t x, y;

	uint32_t possible_crtcs;
	uint32_t gamma_size;
} drmModePlane;
```

---

## Object properties

```c
drmModeObjectProperties *props =
	drmModeObjectGetProperties(drm_fd, crtc_id, DRM_MODE_OBJECT_CRTC);
for (uint32_t i = 0; i < props->count_props; i++) {
	uint32_t prop_id = props->props[i];
	drmModePropertyRes *prop = drmModeGetProperty(drm_fd, prop_id);
}
```
--

```c
typedef struct {
	uint32_t count_props;
	uint32_t *props;
	uint64_t *prop_values;
} drmModeObjectProperties;

typedef struct {
	uint32_t prop_id;
	uint32_t flags;
	char name[DRM_PROP_NAME_LEN];
	int count_values;
	uint64_t *values; /* store the blob lengths */
	int count_enums;
	struct drm_mode_property_enum *enums;
	int count_blobs;
	uint32_t *blob_ids; /* store the blob IDs */
} drmModePropertyRes;
```

???

- Object have properties
- We can read them and change (some of) them
- Props have a name, a type and a value

Use drm_info to display some properties

---

## Reading a property

```c
uint64_t get_property_value(int drm_fd, uint32_t object_id,
		uint32_t object_type, const char *prop_name) {
	drmModeObjectProperties *props =
		drmModeObjectGetProperties(drm_fd, object_id, object_type);
	for (uint32_t i = 0; i < props->count_props; i++) {
		drmModePropertyRes *prop = drmModeGetProperty(drm_fd, props->props[i]);
		uint64_t val = props->prop_values[i];
		if (strcmp(prop->name, prop_name) == 0) {
			drmModeFreeProperty(prop);
			drmModeFreeObjectProperties(props);
			return val;
		}
		drmModeFreeProperty(prop);
	}
	abort(); // Oops, property not found
}
```

???

Demo: `02-selectres`

---

## Atomic commits: why do we need them?

Flickering or bad intermediate state

<img src="img/legacy-bad-frame.svg" style="width: 60%"/>

---

## Atomic commits: why do we need them?

Complicated rollback when something goes wrong

<img src="img/legacy-rollback.svg" style="width: 70%"/>

---

## Atomic commits

<img src="img/atomic.svg"/>
--

In practice: list of (object, property, value)

???

In practice: list of properties to set on objects

---

## Setting a property

```c
drmSetClientCap(drm_fd, DRM_CLIENT_CAP_ATOMIC, 1);

drmModeAtomicReq *req = drmModeAtomicAlloc();
drmModeAtomicAddProperty(req, object_id, prop_id, value);
// Add more properties…

drmModeAtomicCommit(drm_fd, req, flags, NULL);
```
--

```c
void add_property(int drm_fd, drmModeAtomicReq *req, uint32_t object_id,
		uint32_t object_type, const char *prop_name, uint64_t value) {
	uint32_t prop_id = 0;
	drmModeObjectProperties *props =
		drmModeObjectGetProperties(drm_fd, object_id, object_type);
	for (uint32_t i = 0; i < props->count_props; i++) {
		drmModePropertyRes *prop = drmModeGetProperty(drm_fd, props->props[i]);
		if (strcmp(prop->name, prop_name) == 0) {
			prop_id = prop->prop_id;
			break;
		}
	}
	assert(prop_id != 0);

	drmModeAtomicAddProperty(req, object_id, prop_id, value);
}
```

---

## Displaying a buffer

```c
drmModeAtomicReq *req = drmModeAtomicAlloc();

add_property(drm_fd, req, plane_id, DRM_MODE_OBJECT_PLANE, "FB_ID", fb_id);
add_property(drm_fd, req, plane_id, DRM_MODE_OBJECT_PLANE, "SRC_X", 0);
add_property(drm_fd, req, plane_id, DRM_MODE_OBJECT_PLANE, "SRC_Y", 0);
add_property(drm_fd, req, plane_id, DRM_MODE_OBJECT_PLANE, "SRC_W", width << 16);
add_property(drm_fd, req, plane_id, DRM_MODE_OBJECT_PLANE, "SRC_H", height << 16);
add_property(drm_fd, req, plane_id, DRM_MODE_OBJECT_PLANE, "CRTC_X", 0);
add_property(drm_fd, req, plane_id, DRM_MODE_OBJECT_PLANE, "CRTC_Y", 0);
add_property(drm_fd, req, plane_id, DRM_MODE_OBJECT_PLANE, "CRTC_W", width);
add_property(drm_fd, req, plane_id, DRM_MODE_OBJECT_PLANE, "CRTC_H", height);

uint32_t flags = DRM_MODE_ATOMIC_NONBLOCK;
int ret = drmModeAtomicCommit(drm_fd, req, flags, NULL);
if (ret != 0) {
	perror("drmModeAtomicCommit failed");
}
```

???

Demo

---

class: center, middle, inverse

# 3. Goodies

---

## Encoders

All CTRTs aren't compatible with all connectors

```c
typedef struct {
	uint32_t encoder_id;
	uint32_t encoder_type;
	uint32_t crtc_id;
	uint32_t possible_crtcs;
	uint32_t possible_clones;
} drmModeEncoder;
```

<img src="img/pipeline-encoder.svg"/>

???

- So far we've relied on the previous DRM client to correctly set up the CRTC
- Encoders were a mistake

---

## Picking a CRTC and an encoder

```c
uint32_t pick_crtc(int drm_fd, drmModeRes *res, drmModeConnector *conn) {
	// Build a bitmask of all CRTCs we can use for this connector
	uint32_t possible_crtcs = 0;
	for (int i = 0; i < conn->count_encoders; i++) {
		drmModeEncoder *enc = drmModeGetEncoder(drm_fd, conn->encoders[i]);
		possible_crtcs |= enc->possible_crtcs;
		drmModeFreeEncoder(enc);
	}
	assert(possible_crtcs != 0);

	for (int i = 0; i < res->count_crtcs; i++) {
		// Check the CRTC is compatible with the connector
		uint32_t crtc_bit = 1 << i;
		if ((possible_crtcs & crtc_bit) == 0) {
			continue;
		}

		return res->crtcs[i];
	}
	abort(); // Oops, didn't find any suitable CRTC
}
```

---

## Performing a modeset

```c
// Pick the first mode
drmModeModeInfo *mode = &connector->modes[0];

// Create a blob for the mode
uint32_t mode_id = 0;
drmModeCreatePropertyBlob(drm_fd, mode, sizeof(*mode), &mode_id);

// Submit an atomic commit with ALLOW_MODESET
drmModeAtomicReq *req = drmModeAtomicAlloc();

add_property(drm_fd, req, plane_id, DRM_MODE_OBJECT_PLANE, "CRTC_ID", crtc_id);
add_property(drm_fd, req, crtc_id, DRM_MODE_OBJECT_CRTC, "MODE_ID", mode_id);
add_property(drm_fd, req, crtc_id, DRM_MODE_OBJECT_CRTC, "active", 1);
add_property(drm_fd, req, connector_id, DRM_MODE_OBJECT_CONNECTOR, "CRTC_ID", crtc_id);

uint32_t flags = DRM_MODE_ATOMIC_NONBLOCK | DRM_MODE_ATOMIC_ALLOW_MODESET;
drmModeAtomicCommit(drm_fd, req, flags, NULL);
```

---

## Page-flip events

Perform atomic commits with the `DRM_MODE_PAGE_FLIP_EVENT` flag.

```c
// Define a page-flip callback
void handle_page_flip(int drm_fd, unsigned int seq, unsigned int tv_sec,
		unsigned int tv_usec, unsigned int crtc_id, void *data) {
	printf("Got a page-flip event for CRTC %u!\n", crtc_id);
}

while (true) {
	// Wait for a DRM event
	struct pollfd fds[] = {
		{ .fd = drm_fd, .events = POLLIN },
	};
	nfds_t fds_len = 1;
	poll(fds, fds_len, -1);
	assert((fds[0].events & POLLIN) != 0);

	// Read the DRM event
	drmEventContext context = {
		.version = 3,
		.page_flip_handler2 = handle_page_flip,
	};
	drmHandleEvent(drm_fd, &context);
}
```

---

## Double-buffering

- Allocate two buffers: a back-buffer and a front-buffer
- Write to the back-buffer while the kernel uses the front-buffer
- Synchronize writes with page-flip events
- Swap buffers when we're doing writing

???

Not getting into details here:

- OpenGL/GBM can help
- See ascent12's article for more info

---

## Hotplug events

```c
// Create a udev monitor
struct udev *udev = udev_new();
struct udev_monitor *monitor = udev_monitor_new_from_netlink(udev, "udev");
udev_monitor_filter_add_match_subsystem_devtype(monitor, "drm", NULL);
udev_monitor_enable_receiving(monitor);

while (true) {
	// Wait for a udev event
	struct pollfd fds[] = {
		{ .fd = udev_monitor_get_fd(monitor), .events = POLLIN },
	};
	nfds_t fds_len = 1;
	poll(fds, fds_len, -1);
	assert((fds[0].events & POLLIN) != 0);

	// Receive a udev event
	struct udev_device *dev = udev_monitor_receive_device(monitor);
	const char *action = udev_device_get_action(dev);
	if (action != NULL && strcmp(action, "change") == 0) {
		printf("Got udev change event for device %s\n", udev_device_get_sysname(dev));

		// Reload KMS state
	}
}
```

---

## Test-only atomic commits

Allows to check hardware capabilities

```c
uint32_t flags = DRM_MODE_ATOMIC_TEST_ONLY;
int ret = drmModeAtomicCommit(drm_fd, req, flags, NULL);
if (ret == -EINVAL || ret == -ERANGE) {
	printf("Hardware doesn't support this configuration\n");
} else if (ret == 0) {
	printf("Hardware does support this configuration\n");
} else {
	perror("drmModeAtomicCommit failed");
}
```

---

## OpenGL integration: display creation

```c
struct gbm_device *gbm_device = gbm_create_device(drm_fd);

// Create a new EGL display (requires EGL_KHR_platform_gbm)
EGLDisplay egl_display =
	eglGetPlatformDisplay(EGL_PLATFORM_GBM_KHR, gbm_device, NULL);
```

---

## OpenGL integration: context creation

```c
// Choose an EGL config
EGLint configs_len = 0;
eglGetConfigs(egl_display, NULL, 0, &configs_len);
EGLConfig *configs = malloc(configs_len * sizeof(EGLConfig));
eglGetConfigs(egl_display, configs, configs_len, &configs_len);

EGLConfig egl_config = EGL_NO_CONFIG_KHR;
for (EGLint i = 0; i < configs_len; i++) {
	EGLint visual = DRM_FORMAT_INVALID;
	eglGetConfigAttrib(egl_display, configs[i], EGL_NATIVE_VISUAL_ID, &visual);
	if (visual == DRM_FORMAT_XRGB8888) {
		egl_config = configs[i];
		break;
	}
}
free(configs);
assert(egl_config != EGL_NO_CONFIG_KHR);

// Create a new EGL context
EGLint attribs[] = { EGL_CONTEXT_CLIENT_VERSION, 2, EGL_NONE }; // GL ES 2.0
EGLContext egl_ctx = eglCreateContext(egl_display, egl_config, EGL_NO_CONTEXT, attribs);
```

---

## OpenGL integration: surface creation

```c
uint32_t format = DRM_FORMAT_XRGB8888;
uint32_t flags = GBM_BO_USE_SCANOUT | GBM_BO_USE_RENDERING;
struct gbm_surface *gbm_surface =
	gbm_surface_create(gbm_device, width, height, format, flags);

// Create a new EGL surface (requires EGL_KHR_platform_gbm)
EGLSurface egl_surface =
	eglCreatePlatformWindowSurface(egl_display, egl_config, gbm_surface, NULL);
```

---

## OpenGL integration: rendering loop

```c
// Draw with OpenGL as usual…

eglSwapBuffers(egl_display, egl_surface);

struct gbm_bo *gbm_bo = gbm_surface_lock_front_buffer(gbm_surface);

uint32_t handles[4] = { gbm_bo_get_handle(gbm_bo).u32 };
uint32_t strides[4] = { gbm_bo_get_stride(gbm_bo) };
uint32_t offsets[4] = { 0 };
uint32_t fb_id = 0;
drmModeAddFB2(drm_fd, width, height, DRM_FORMAT_XRGB8888,
	handles, strides, offsets, &fb_id, 0);

// Perform an atomic commit with fb_id…

drmModeRmFB(drm_fd, fb_id);
```

```c
gbm_surface_release_buffer(gbm_surface, gbm_bo);
```

???

- BO cannot be released before it becomes off-screen (replaced by another BO)
- Lifecycle: queued → on-screen → off-screen

---

## Next steps

- Set bitfield/enum properties
- System seats, libseat
- Format modifiers
- Use multiple planes
- libliftoff

---

## References

* Demos for this talk: https://git.sr.ht/~emersion/foss-north-kms
* ascent12's drm_doc: https://github.com/ascent12/drm_doc
* daniels' kms-quads: https://gitlab.freedesktop.org/daniels/kms-quads
* drm_info: https://github.com/ascent12/drm_info
* DRM database: https://drmdb.emersion.fr/
* Discuss in #dri-devel on Freenode

---

class: center, middle, inverse

# Thanks!

Questions?


<style>
img {
  max-width: 100%;
  max-height: 80%;
}

.new {
  color: #2306fb;
}

.inverse {
  background: #272822;
  color: #777872;
  text-shadow: 0 0 20px #333;
}
.inverse h1, .inverse h2 {
  color: #f3f3f3;
  line-height: 0.8em;
}

.column:nth-of-type(1) {float:left}
.column:nth-of-type(2) {float:right}

.split-50 .column:nth-of-type(1) {width: 47%}
.split-50 .column:nth-of-type(2) {width: 47%}
</style>
