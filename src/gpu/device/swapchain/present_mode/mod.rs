mod from_vk;
mod into_vk;

/// The type of presentation mode a swapchain can use
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapchainPresentMode {
    /// The presentation engine does not wait for a vertical blanking period to update the current
    /// image. This may result in tearing.
    Immediate,

    /// The presentation engine waits for the next vertical blanking period to update the current
    /// image. Tearing cannot be observed. An internal single-entry queue is used to hold pending
    /// presentation requests. If the queue is full when a new presentation request is received,
    /// the new request replaces the existing entry, and any images associated with the prior entry
    /// become available for reuse by the application. One request is removed from the queue and
    /// processed during each vertical blanking period in which the queue is non-empty.
    Mailbox,

    /// The presentation engine waits for the next vertical blanking period to update the current
    /// image. Tearing cannot be observed. An internal queue is used to hold pending presentation
    /// requests. New requests are appended to the end of the queue, and one request is removed
    /// from the beginning of the queue and processed during each vertical blanking period in which
    /// the queue is non-empty.
    Fifo,

    /// The presentation engine generally waits for the next vertical blanking period to update the
    /// current image. If a vertical blanking period has already passed since the last update of
    /// the current image then the presentation engine does not wait for another vertical blanking
    /// period for the update, meaning this mode may result in visible tearing in this case. This
    /// mode is useful for reducing visual stutter with an application that will mostly present a
    /// new image before the next vertical blanking period, but may occasionally be late, and
    /// present a new image just after the next vertical blanking period. An internal queue is used
    /// to hold pending presentation requests. New requests are appended to the end of the queue,
    /// and one request is removed from the beginning of the queue and processed during or after
    /// each vertical blanking period in which the queue is non-empty.
    FifoRelaxed,
}
