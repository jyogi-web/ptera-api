use crate::{service_handler::DEFAULT_RATE, CONFIG};

pub(crate) fn rate_calculation(win: u64, lose: u64) -> (u64, u64) {
    let win = win as f64;
    let lose = lose as f64;
    let (update_win, update_lose) = if win >= lose {
        // 格上が勝った場合
        let delta_win = ((lose.log2() + 50.0) as u64).min(CONFIG.max_delta_rate);
        let delta_lose = ((win.log2() + 20.0) as u64).min(CONFIG.max_delta_rate);

        let win = win as u64;
        let lose = lose as u64;
        (
            win.saturating_add(delta_win),
            lose.saturating_sub(delta_lose),
        )
    } else {
        // 格下が勝った場合
        let delta_win = ((lose.powi(4).log2() + (win - lose).abs().log2() + 50.0) as u64)
            .min(CONFIG.max_delta_rate);
        let delta_lose = ((win.powi(2).log2() + (win - lose).abs().log2() + 20.0) as u64)
            .min(CONFIG.max_delta_rate);

        let win = win as u64;
        let lose = lose as u64;
        (
            win.saturating_add(delta_win),
            lose.saturating_sub(delta_lose),
        )
    };

    (update_win, update_lose)
}
