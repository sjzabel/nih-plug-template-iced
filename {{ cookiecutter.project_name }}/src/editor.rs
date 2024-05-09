use nih_plug::prelude::{Editor, GuiContext};
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::*;
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;

use crate::{{ cookiecutter.struct_name }}Params;

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(400, 500)
}

pub(crate) fn create(
    params: Arc<{{ cookiecutter.struct_name }}Params>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<{{ cookiecutter.struct_name }}Editor>(editor_state, params)
}

struct {{ cookiecutter.struct_name }}Editor {
    params: Arc<{{ cookiecutter.struct_name }}Params>,
    context: Arc<dyn GuiContext>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    /// Update a parameter's value.
    ParamUpdate(nih_widgets::ParamMessage),
}

impl IcedEditor for {{ cookiecutter.struct_name }}Editor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = Arc<{{ cookiecutter.struct_name }}Params>;

    fn new(
        params: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = {{ cookiecutter.struct_name }}Editor {
            params,
            context,
        };

        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::ParamUpdate(message) => self.handle_param_message(message),
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        // Cast the settings to the correct type
        let tempo = self.params.tempo.load(Relaxed);
        let tempo_text = format!("Host BPMs: {:.2}", tempo);

        let playing = self.params.playing.load(Relaxed);
        let playing_text = format!("Playing: {}", playing);

        let preroll_active = self.params.playing.load(Relaxed);
        let preroll_active_text = format!("Pre-Roll Active: {}", playing);

        let recording = self.params.recording.load(Relaxed);
        let recording_text = format!("Recording: {}", recording);

        let time_sig_denominator = self.params.time_sig_denominator.load(Relaxed);
        let time_sig_numerator = self.params.time_sig_numerator.load(Relaxed);
        let time_sig_text = format!("Time Signature: {}/{}", time_sig_numerator, time_sig_denominator);
    
        let pos_samples = self.params.pos_samples.load(Relaxed);
        let pos_samples_text = format!("pos_samples: {}", pos_samples);

        let pos_seconds = self.params.pos_seconds.load(Relaxed);
        let pos_seconds_text = format!("pos_seconds: {:.2}", pos_seconds);

        let pos_beats = self.params.pos_beats.load(Relaxed);
        let pos_beats_text = format!("pos_beats: {:.2}", pos_beats);

        let bar_start_pos_beats = self.params.bar_start_pos_beats.load(Relaxed);
        let bar_start_pos_beats_text = format!("bar_start_pos_beats: {:.2}", bar_start_pos_beats);

        let bar_number = self.params.bar_number.load(Relaxed);
        let bar_number_text = format!("bar_number: {}", bar_number);

        let loop_range_samples_start = self.params.loop_range_samples_start.load(Relaxed);
        let loop_range_samples_end = self.params.loop_range_samples_end.load(Relaxed);
        let loop_range_samples_text = format!("Loop Samples: {} -> {}", loop_range_samples_start, loop_range_samples_end);

        let loop_range_seconds_start = self.params.loop_range_seconds_start.load(Relaxed);
        let loop_range_seconds_end = self.params.loop_range_seconds_end.load(Relaxed);
        let loop_range_seconds_text = format!("Loop seconds: {:.2} -> {:.2}", loop_range_seconds_start, loop_range_seconds_end);

        let loop_range_beats_start = self.params.loop_range_beats_start.load(Relaxed);
        let loop_range_beats_end = self.params.loop_range_beats_end.load(Relaxed);
        let loop_range_beats_text = format!("Loop beats: {:.2} -> {:.2}", loop_range_beats_start, loop_range_beats_end);


        Column::new()
            .align_items(Alignment::Center)
            .push(
                Text::new("Host Settings")
                    .font(assets::NOTO_SANS_LIGHT)
                    .size(40)
                    .height(50.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Bottom),
            )
            .push(Space::with_height(10.into()))
            .push(
                Text::new(tempo_text)
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(Space::with_height(10.into()))
            .push(
                Text::new(playing_text)
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(Space::with_height(10.into()))
            .push(
                Text::new(preroll_active_text)
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(Space::with_height(10.into()))
            .push(
                Text::new(recording_text)
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(Space::with_height(10.into()))
            .push(
                Text::new(time_sig_text)
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(Space::with_height(10.into()))
            .push(
                Text::new(pos_samples_text)
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(Space::with_height(10.into()))
            .push(
                Text::new(pos_seconds_text)
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(Space::with_height(10.into()))
            .push(
                Text::new(pos_beats_text)
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(Space::with_height(10.into()))
            .push(
                Text::new(bar_start_pos_beats_text)
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(Space::with_height(10.into()))
            .push(
                Text::new(bar_number_text)
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(Space::with_height(10.into()))
            .push(
                Text::new(loop_range_samples_text)
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(Space::with_height(10.into()))
            .push(
                Text::new(loop_range_seconds_text)
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(Space::with_height(10.into()))
            .push(
                Text::new(loop_range_beats_text)
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .into()
    }

    fn background_color(&self) -> nih_plug_iced::Color {
        nih_plug_iced::Color {
            r: 0.98,
            g: 0.98,
            b: 0.98,
            a: 1.0,
        }
    }
}
