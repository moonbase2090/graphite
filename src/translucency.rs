//! Pure Graphite translucency role policy data.

/// Roles that participate in the Graphite opacity policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranslucencyRole {
    /// Regular and emphasized text.
    Text,
    /// The terminal cursor.
    Cursor,
    /// Status dots.
    StatusDots,
    /// The `needs you` badge.
    NeedsYouBadge,
    /// The active tab chip.
    ActiveTabChip,
    /// The command field.
    CommandField,
    /// The pane focus ring.
    FocusRing,
    /// Window ground.
    WindowGround,
    /// Pane surfaces.
    PaneSurfaces,
    /// Tabs and spaces bar backgrounds.
    BarBackgrounds,
}

/// Alpha policy assigned to a translucency role.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphaPolicy {
    /// Always rendered opaque.
    Opaque,
    /// Follows the host's chrome-alpha setting.
    FollowChromeAlpha,
}

/// Returns every role exactly once in the plan's policy-table order.
pub fn translucency_roles() -> &'static [TranslucencyRole; 10] {
    todo!()
}

/// Returns the data-only alpha policy for a role.
pub fn alpha_policy(role: TranslucencyRole) -> AlphaPolicy {
    let _ = role;
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPAQUE: [TranslucencyRole; 7] = [
        TranslucencyRole::Text,
        TranslucencyRole::Cursor,
        TranslucencyRole::StatusDots,
        TranslucencyRole::NeedsYouBadge,
        TranslucencyRole::ActiveTabChip,
        TranslucencyRole::CommandField,
        TranslucencyRole::FocusRing,
    ];
    const FOLLOW: [TranslucencyRole; 3] = [
        TranslucencyRole::WindowGround,
        TranslucencyRole::PaneSurfaces,
        TranslucencyRole::BarBackgrounds,
    ];

    #[test]
    fn every_brief_role_maps_to_the_literal_policy_set() {
        for role in OPAQUE {
            assert_eq!(alpha_policy(role), AlphaPolicy::Opaque);
        }
        for role in FOLLOW {
            assert_eq!(alpha_policy(role), AlphaPolicy::FollowChromeAlpha);
        }
    }

    #[test]
    fn role_table_covers_each_literal_role_exactly_once() {
        let expected = [
            OPAQUE[0], OPAQUE[1], OPAQUE[2], OPAQUE[3], OPAQUE[4], OPAQUE[5], OPAQUE[6], FOLLOW[0],
            FOLLOW[1], FOLLOW[2],
        ];
        let actual = translucency_roles();
        assert_eq!(actual, &expected);
        for (index, role) in actual.iter().enumerate() {
            assert_eq!(
                actual.iter().filter(|candidate| *candidate == role).count(),
                1
            );
            assert_eq!(
                alpha_policy(*role),
                if index < OPAQUE.len() {
                    AlphaPolicy::Opaque
                } else {
                    AlphaPolicy::FollowChromeAlpha
                }
            );
        }
    }
}
