use atomic_float::AtomicF64;
use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::AtomicI64;

// This is a mostly gutted version of the nih_plug_iced tutorial check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain_gui_iced/src/lib.rs to get
// started

mod editor;

struct {{ cookiecutter.struct_name }} {
    params: Arc<{{ cookiecutter.struct_name }}Params>,
}

#[derive(Params)]
struct {{ cookiecutter.struct_name }}Params {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,

    tempo: Arc<AtomicF64>,
    playing: Arc<AtomicBool>,
    preroll_active: Arc<AtomicBool>,
    recording: Arc<AtomicBool>,
    time_sig_denominator: Arc<AtomicI32>,
    time_sig_numerator: Arc<AtomicI32>,
    pos_samples: Arc<AtomicI64>,
    pos_seconds: Arc<AtomicF64>,
    pos_beats: Arc<AtomicF64>,
    bar_start_pos_beats: Arc<AtomicF64>,
    bar_number: Arc<AtomicI32>,
    loop_range_samples_start: Arc<AtomicI64>,
    loop_range_samples_end: Arc<AtomicI64>,
    loop_range_seconds_start: Arc<AtomicF64>,
    loop_range_seconds_end: Arc<AtomicF64>,
    loop_range_beats_start: Arc<AtomicF64>,
    loop_range_beats_end: Arc<AtomicF64>,
}

impl Default for {{ cookiecutter.struct_name }} {
    fn default() -> Self {
        Self {
            params: Arc::new({{ cookiecutter.struct_name }}Params::default()),
        }
    }
}

impl Default for {{ cookiecutter.struct_name }}Params {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            // Host parameters to pass to the GUI
            tempo: Arc::new(AtomicF64::new(120.0)),
            playing: Arc::new(AtomicBool::new(false)),
            preroll_active: Arc::new(AtomicBool::new(false)),
            recording: Arc::new(AtomicBool::new(false)),
            time_sig_denominator: Arc::new(AtomicI32::new(4)),
            time_sig_numerator: Arc::new(AtomicI32::new(4)),
            pos_samples: Arc::new(AtomicI64::new(0)),
            pos_seconds: Arc::new(AtomicF64::new(0.0)),
            pos_beats: Arc::new(AtomicF64::new(0.0)),
            bar_start_pos_beats: Arc::new(AtomicF64::new(0.0)),
            bar_number: Arc::new(AtomicI32::new(0)),
            loop_range_samples_start: Arc::new(AtomicI64::new(0)),
            loop_range_samples_end: Arc::new(AtomicI64::new(0)),
            loop_range_seconds_start: Arc::new(AtomicF64::new(0.0)),
            loop_range_seconds_end: Arc::new(AtomicF64::new(0.0)),
            loop_range_beats_start: Arc::new(AtomicF64::new(0.0)),
            loop_range_beats_end: Arc::new(AtomicF64::new(0.0)),
    }
}

impl Plugin for {{ cookiecutter.struct_name }} {
    const NAME: &'static str = "{{ cookiecutter.plugin_name }}";
    const VENDOR: &'static str = "{{ cookiecutter.author }}";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "{{ cookiecutter.email_address }}";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.
        names: PortNames::const_default(),
    }];


    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            for sample in channel_samples {}

            // To save resources, a plugin can (and probably should!) only perform expensive
            // calculations that are only displayed on the GUI while the GUI is open
            if self.params.editor_state.is_open() {
                if let Some(tempo) = context.transport().tempo {
                    self.params.tempo.store(tempo, Relaxed);
                }

                self.params.playing.store(context.transport().playing, Relaxed);
                if let Some(preroll_active) = context.transport().preroll_active {
                    self.params.preroll_active.store(preroll_active, Relaxed);
                }

                self.params.recording.store(context.transport().recording, Relaxed);

                if let Some(time_sig_denominator) = context.transport().time_sig_denominator {
                    self.params.time_sig_denominator.store(time_sig_denominator, Relaxed);
                }
                if let Some(time_sig_numerator) = context.transport().time_sig_numerator {
                    self.params.time_sig_numerator.store(time_sig_numerator, Relaxed);
                }
                if let Some(pos_samples) = context.transport().pos_samples() {
                    self.params.pos_samples.store(pos_samples, Relaxed);
                }
                if let Some(pos_seconds) = context.transport().pos_seconds() {
                    self.params.pos_seconds.store(pos_seconds, Relaxed);
                }
                if let Some(pos_beats) = context.transport().pos_beats() {
                    self.params.pos_beats.store(pos_beats, Relaxed);
                }
                if let Some(bar_start_pos_beats) = context.transport().bar_start_pos_beats() {
                    self.params.bar_start_pos_beats.store(bar_start_pos_beats, Relaxed);
                }
                if let Some(bar_number) = context.transport().bar_number() {
                    self.params.bar_number.store(bar_number, Relaxed);
                }
                if let Some((loop_range_samples_start,loop_range_samples_end)) = context.transport().loop_range_samples() {
                    self.params.loop_range_samples_start.store(loop_range_samples_start, Relaxed);
                    self.params.loop_range_samples_end.store(loop_range_samples_end, Relaxed);
                }
                if let Some((loop_range_seconds_start,loop_range_seconds_end)) = context.transport().loop_range_seconds() {
                    self.params.loop_range_seconds_start.store(loop_range_seconds_start, Relaxed);
                    self.params.loop_range_seconds_end.store(loop_range_seconds_end, Relaxed);
                }
                if let Some((loop_range_beats_start,loop_range_beats_end)) = context.transport().loop_range_beats() {
                    self.params.loop_range_beats_start.store(loop_range_beats_start, Relaxed);
                    self.params.loop_range_beats_end.store(loop_range_beats_end, Relaxed);
                }
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for {{ cookiecutter.struct_name }} {
    const CLAP_ID: &'static str = "{{ cookiecutter.clap_id }}";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("{{ cookiecutter.description }}");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        // ClapFeature::AudioEffect,
        // ClapFeature::Stereo,
        // ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for {{ cookiecutter.struct_name }} {
    const VST3_CLASS_ID: [u8; 16] = *b"{{ cookiecutter.vst3_id }}";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!({{ cookiecutter.struct_name }});
nih_export_vst3!({{ cookiecutter.struct_name }});
