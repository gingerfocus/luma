use std::path::PathBuf;

#[derive(Debug)]
pub struct Args {
    /// The input file
    pub input: PathBuf,
    /// Enables logging
    pub log: bool,
    /// Path where log files should be written
    pub file: Option<PathBuf>,
}

pub fn parse() -> Args {
    let mut args = std::env::args();
    _ = args.next(); // skip the first argument (the program name)

    let mut log = false;
    let mut input = None;
    let mut file = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-l" | "--log" => log = true,
            "-L" | "--log-file" => {
                let f = args.next().unwrap();
                file = Some(PathBuf::from(f));
            }
            _ => input = Some(PathBuf::from(arg)),
        }
    }

    if let Some(input) = input {
        Args { input, log, file }
    } else {
        // show help
        std::process::exit(1);
    }
}

// mod thinking {
//     use std::time::Duration;
//
//     use glam::u32::UVec3;
//     use rand::Rng;
//
//     // const charCyclingFPS: Second = 1.0 / 22.0;
//     const MAX_CYCLING_CHARS: u8 = 120;
//
//     const RUNES: &str = "0123456789abcdefABCDEF~!@#$£€%^&*()+=_";
//
//     // &["", ".", "..", "..."]
//
//     struct Spinner {
//         frames: Vec<&'static str>,
//         fps: f64,
//     }
//
//     // cyclingChar is a single animated character.
//     struct CyclingChar {
//         final_value: String,
//         current_value: Vec<Rune>,
//         initial_delay: Duration,
//         lifetime: Duration,
//     }
//
//     struct Rune {
//         character: char,
//         color: UVec3,
//     }
//
//     impl Rune {
//         fn random() -> Rune {
//             let mut rng = rand::thread_rng();
//             let n = rng.gen_range(0..RUNES.len());
//             let character = RUNES.chars().nth(n).unwrap();
//             Rune {
//                 character,
//                 color: UVec3::ZERO,
//             }
//         }
//     }
//
//     // fn state(start: Second, c: &CyclingChar) -> char {
//     //     let now = Second(0.0);
//     //     if now < start + c.initial_delay {
//     //         return '.';
//     //     }
//     //     if c.final_value > 0 && now > start + c.initial_delay {
//     //         return c.final_value;
//     //     }
//     //     return random_rune();
//     // }
//
//     // func stepChars() tea.Cmd {
//     // 	return tea.Tick(charCyclingFPS, func(_ time.Time) tea.Msg {
//     // 		return stepCharsMsg{}
//     // 	})
//     // }
//     //
//     // // cyclingChars is the model that manages the animation that displays while the
//     // // output is being generated.
//     // type cyclingChars struct {
//     // 	start           time.Time
//     // 	chars           []cyclingChar
//     // 	ramp            []lipgloss.Style
//     // 	label           []rune
//     // 	ellipsis        spinner.Model
//     // 	ellipsisStarted bool
//     // 	styles          styles
//     // }
//     //
//     // func newCyclingChars(initialCharsSize uint, label string, r *lipgloss.Renderer, s styles) cyclingChars {
//     // 	n := int(initialCharsSize)
//     // 	if n > maxCyclingChars {
//     // 		n = maxCyclingChars
//     // 	}
//     //
//     // 	gap := " "
//     // 	if n == 0 {
//     // 		gap = ""
//     // 	}
//     //
//     // 	c := cyclingChars{
//     // 		start:    time.Now(),
//     // 		label:    []rune(gap + label),
//     // 		ellipsis: spinner.New(spinner.WithSpinner(ellipsisSpinner)),
//     // 		styles:   s,
//     // 	}
//     //
//     // 	// If we're in truecolor mode (and there are enough cycling characters)
//     // 	// color the cycling characters with a gradient ramp.
//     // 	const minRampSize = 3
//     // 	if n >= minRampSize && r.ColorProfile() == termenv.TrueColor {
//     // 		c.ramp = make([]lipgloss.Style, n)
//     // 		ramp := makeGradientRamp(n)
//     // 		for i, color := range ramp {
//     // 			c.ramp[i] = r.NewStyle().Foreground(color)
//     // 		}
//     // 	}
//     //
//     // 	makeDelay := func(a int32, b time.Duration) time.Duration {
//     // 		return time.Duration(rand.Int31n(a)) * (time.Millisecond * b) //nolint:gosec
//     // 	}
//     //
//     // 	makeInitialDelay := func() time.Duration {
//     // 		return makeDelay(8, 60) //nolint:gomnd
//     // 	}
//     //
//     // 	c.chars = make([]cyclingChar, n+len(c.label))
//     //
//     // 	// Initial characters that cycle forever.
//     // 	for i := 0; i < n; i++ {
//     // 		c.chars[i] = cyclingChar{
//     // 			finalValue:   -1, // cycle forever
//     // 			initialDelay: makeInitialDelay(),
//     // 		}
//     // 	}
//     //
//     // 	// Label text that only cycles for a little while.
//     // 	for i, r := range c.label {
//     // 		c.chars[i+n] = cyclingChar{
//     // 			finalValue:   r,
//     // 			initialDelay: makeInitialDelay(),
//     // 			lifetime:     makeDelay(5, 180), //nolint:gomnd
//     // 		}
//     // 	}
//     //
//     // 	return c
//     // }
//     //
//     // // Init initializes the animation.
//     // func (c cyclingChars) Init() tea.Cmd {
//     // 	return stepChars()
//     // }
//     //
//     // // Update handles messages.
//     // func (c cyclingChars) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
//     // 	var cmd tea.Cmd
//     // 	switch msg.(type) {
//     // 	case stepCharsMsg:
//     // 		for i, char := range c.chars {
//     // 			switch char.state(c.start) {
//     // 			case charInitialState:
//     // 				c.chars[i].currentValue = '.'
//     // 			case charCyclingState:
//     // 				c.chars[i].currentValue = char.randomRune()
//     // 			case charEndOfLifeState:
//     // 				c.chars[i].currentValue = char.finalValue
//     // 			}
//     // 		}
//     //
//     // 		if !c.ellipsisStarted {
//     // 			var eol int
//     // 			for _, char := range c.chars {
//     // 				if char.state(c.start) == charEndOfLifeState {
//     // 					eol++
//     // 				}
//     // 			}
//     // 			if eol == len(c.label) {
//     // 				// If our entire label has reached end of life, start the
//     // 				// ellipsis "spinner" after a short pause.
//     // 				c.ellipsisStarted = true
//     // 				cmd = tea.Tick(time.Millisecond*220, func(_ time.Time) tea.Msg { //nolint:gomnd
//     // 					return c.ellipsis.Tick()
//     // 				})
//     // 			}
//     // 		}
//     //
//     // 		return c, tea.Batch(stepChars(), cmd)
//     // 	case spinner.TickMsg:
//     // 		var cmd tea.Cmd
//     // 		c.ellipsis, cmd = c.ellipsis.Update(msg)
//     // 		return c, cmd
//     // 	default:
//     // 		return c, nil
//     // 	}
//     // }
//     //
//     // // View renders the animation.
//     // func (c cyclingChars) View() string {
//     // 	var b strings.Builder
//     // 	for i, char := range c.chars {
//     // 		var (
//     // 			s *lipgloss.Style
//     // 			r = char.currentValue
//     // 		)
//     // 		if len(c.ramp) > 0 && i < len(c.ramp) {
//     // 			// There's a gradient ramp style defined for this char. Style it
//     // 			// accordingly.
//     // 			s = &c.ramp[i]
//     // 		} else if char.finalValue < 0 {
//     // 			// No gradient ramp defined, but this color will cycle forever so
//     // 			// let's color it accordingly.
//     // 			s = &c.styles.cyclingChars
//     // 		}
//     // 		if s != nil {
//     // 			b.WriteString(s.Render(string(r)))
//     // 			continue
//     // 		}
//     // 		b.WriteRune(r)
//     // 	}
//     // 	return b.String() + c.ellipsis.View()
//     // }
//
//     // 	const startColor = "#F967DC"
//     // 	const endColor = "#6B50FF"
//
//     fn make_gradient(start: UVec3, end: UVec3, steps: u32) -> Vec<UVec3> {
//         if steps < 3 {
//             panic!("Gradients need at least 3 steps");
//         }
//
//         let diff = start - end;
//         let components = diff.to_array();
//         let step_size = steps - 2;
//         let step = UVec3::new(
//             components[0] / step_size,
//             components[1] / step_size,
//             components[2] / step_size,
//         );
//
//         let mut sum = start.clone();
//
//         let mut buf = Vec::new();
//         buf.push(start);
//
//         for _ in 0..=step_size {
//             sum += step;
//             buf.push(sum.clone());
//         }
//         buf.push(end);
//         buf
//     }
// }
