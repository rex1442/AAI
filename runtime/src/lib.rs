impl pallet_video_metadata::Config for Runtime {
    type Event = Event;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // ... 기존 팔레트들
        VideoMetadata: pallet_video_metadata::{Pallet, Call, Storage, Event<T>},
    }
);
