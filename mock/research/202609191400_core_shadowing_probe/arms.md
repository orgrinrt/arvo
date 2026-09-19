<!-- Written by run.sh in the same run as run.out. Not edited by hand. -->

| arm | what it checks | expected | observed |
| --- | --- | --- | --- |
| `` a_mod_core_shadows_the_bare_path.rs `` | 1 const assert; reading the fake width: item 1 | `` builds `` | builds |
| `` a_mod_core_shadows_the_bare_path.rs ``, item 1 flipped | const assert: `` const _: () = assert!(<CHECK as F>::MAX != 255, "the shadow did not win"); `` | refused by that item alone | refused |
| `` a_mod_core_shadows_the_bare_path.rs ``, fake width set to 64 | item 1 reading the fake width | refused by item 1 alone | refused |
| `` a_use_alias_shadows_the_bare_path.rs `` | 1 const assert; reading the fake width: item 1 | `` builds `` | builds |
| `` a_use_alias_shadows_the_bare_path.rs ``, item 1 flipped | const assert: `` const _: () = assert!(<CHECK as F>::MAX != 255, "the alias shadow did not win"); `` | refused by that item alone | refused |
| `` a_use_alias_shadows_the_bare_path.rs ``, fake width set to 64 | item 1 reading the fake width | refused by item 1 alone | refused |
| `` a_bare_mod_usize_confirms_the_original_premise.rs `` | 1 const assert; reading the fake width: item 1 | `` builds `` | builds |
| `` a_bare_mod_usize_confirms_the_original_premise.rs ``, item 1 flipped | const assert: `` <CHECK as F>::MAX != 255, `` | refused by that item alone | refused |
| `` a_bare_mod_usize_confirms_the_original_premise.rs ``, fake width set to 64 | item 1 reading the fake width | refused by item 1 alone | refused |
| `` the_leading_double_colon_resists_both_shadows.rs `` | 1 const assert; reading past it: item 1 | `` builds `` | builds |
| `` the_leading_double_colon_resists_both_shadows.rs ``, item 1 flipped | const assert: `` <CHECK as F>::MAX != (1i128 << usize::BITS) - 1, `` | refused by that item alone | refused |
| `` the_leading_double_colon_resists_both_shadows.rs ``, fake width set to 64 | no item reading the fake width | builds | builds |
| `` the_leading_double_colon_resists_both_shadows.rs ``, leading `::` dropped | item 1 reading past the fake width | refused by item 1 alone | refused |
| `` the_use_alias_does_not_reach_the_leading_colon_path.rs `` | 1 const assert; reading past it: item 1 | `` builds `` | builds |
| `` the_use_alias_does_not_reach_the_leading_colon_path.rs ``, item 1 flipped | const assert: `` <CHECK as F>::MAX != (1i128 << usize::BITS) - 1, `` | refused by that item alone | refused |
| `` the_use_alias_does_not_reach_the_leading_colon_path.rs ``, fake width set to 64 | no item reading the fake width | builds | builds |
| `` the_use_alias_does_not_reach_the_leading_colon_path.rs ``, leading `::` dropped | item 1 reading past the fake width | refused by item 1 alone | refused |
| `` the_leading_double_colon_resists_an_unaliased_use_of_core.rs `` | 2 const assert; reading the fake width: item 1; reading past it: item 2 | `` builds `` | builds |
| `` the_leading_double_colon_resists_an_unaliased_use_of_core.rs ``, item 1 flipped | const assert: `` core::primitive::usize::BITS != 9, `` | refused by that item alone | refused |
| `` the_leading_double_colon_resists_an_unaliased_use_of_core.rs ``, item 2 flipped | const assert: `` ::core::primitive::usize::BITS != usize::BITS, `` | refused by that item alone | refused |
| `` the_leading_double_colon_resists_an_unaliased_use_of_core.rs ``, fake width set to 64 | item 1 reading the fake width | refused by item 1 alone | refused |
| `` the_leading_double_colon_resists_an_unaliased_use_of_core.rs ``, leading `::` dropped | item 2 reading past the fake width | refused by item 2 alone | refused |
| `` the_leading_double_colon_resists_a_self_under_core.rs `` | 2 const assert; reading the fake width: item 1; reading past it: item 2 | `` builds `` | builds |
| `` the_leading_double_colon_resists_a_self_under_core.rs ``, item 1 flipped | const assert: `` core::primitive::usize::BITS != 9, `` | refused by that item alone | refused |
| `` the_leading_double_colon_resists_a_self_under_core.rs ``, item 2 flipped | const assert: `` ::core::primitive::usize::BITS != usize::BITS, `` | refused by that item alone | refused |
| `` the_leading_double_colon_resists_a_self_under_core.rs ``, fake width set to 64 | item 1 reading the fake width | refused by item 1 alone | refused |
| `` the_leading_double_colon_resists_a_self_under_core.rs ``, leading `::` dropped | item 2 reading past the fake width | refused by item 2 alone | refused |
| `` extern_crate_self_as_core_hijacks_the_absolute_path.rs `` | 1 const assert; reading the fake width: item 1 | `` builds `` | builds |
| `` extern_crate_self_as_core_hijacks_the_absolute_path.rs ``, item 1 flipped | const assert: `` <CHECK as F>::MAX != 255, `` | refused by that item alone | refused |
| `` extern_crate_self_as_core_hijacks_the_absolute_path.rs ``, fake width set to 64 | item 1 reading the fake width | refused by item 1 alone | refused |
| `` the_raw_ident_extern_crate_self_hijacks_the_leading_colon_path.rs `` | 2 const assert; reading the fake width: items 1, 2 | `` builds `` | builds |
| `` the_raw_ident_extern_crate_self_hijacks_the_leading_colon_path.rs ``, item 1 flipped | const assert: `` <CHECK as F>::MAX != 255, `` | refused by that item alone | refused |
| `` the_raw_ident_extern_crate_self_hijacks_the_leading_colon_path.rs ``, item 2 flipped | const assert: `` ::core::primitive::usize::BITS == usize::BITS, `` | refused by that item alone | refused |
| `` the_raw_ident_extern_crate_self_hijacks_the_leading_colon_path.rs ``, fake width set to 64 | items 1, 2 reading the fake width | refused by items 1, 2 alone | refused |
| `` a_macro_expanded_extern_crate_self_as_core_is_refused.rs `` | 1 const assert | `` refused with macro-expanded `extern crate` items cannot shadow names passed with `--extern` `` | refused |
| `` a_macro_expanded_extern_crate_self_as_core_is_refused.rs ``, fake width set to 64 | no item reading the fake width | refused with the same texts | refused |
| `` manifest_rename/user `` | 2 const assert; reading the fake width: items 1, 2 | `` builds `` | builds |
| `` manifest_rename/user ``, item 1 flipped | const assert: `` const _: () = assert!(W != 8, "the manifest rename did not take over `::core`"); `` | refused by that item alone | refused |
| `` manifest_rename/user ``, item 2 flipped | const assert: `` W == usize::BITS, `` | refused by that item alone | refused |
| `` manifest_rename/user ``, fake width set to 64 | items 1, 2 reading the fake width | refused by items 1, 2 alone | refused |
| `` manifest_rename/user_of_the_crate_with_only_the_prelude `` | 2 const assert; reading the fake width: items 1, 2 | `` builds `` | builds |
| `` manifest_rename/user_of_the_crate_with_only_the_prelude ``, item 1 flipped | const assert: `` const _: () = assert!(W != 8, "the manifest rename did not take over `::core`"); `` | refused by that item alone | refused |
| `` manifest_rename/user_of_the_crate_with_only_the_prelude ``, item 2 flipped | const assert: `` W == usize::BITS, `` | refused by that item alone | refused |
| `` manifest_rename/user_of_the_crate_with_only_the_prelude ``, fake width set to 64 | items 1, 2 reading the fake width | refused by items 1, 2 alone | refused |
| `` manifest_rename/user_of_the_crate_with_an_empty_prelude `` | 2 array length; reading the fake width: items 1, 2 | `` builds `` | builds |
| `` manifest_rename/user_of_the_crate_with_an_empty_prelude ``, item 1 flipped | array length: `` const _: [(); 1] = [(); (W != 8) as usize]; `` | refused by that item alone | refused |
| `` manifest_rename/user_of_the_crate_with_an_empty_prelude ``, item 2 flipped | array length: `` const _: [(); 1] = [(); (W == usize::BITS) as usize]; `` | refused by that item alone | refused |
| `` manifest_rename/user_of_the_crate_with_an_empty_prelude ``, fake width set to 64 | items 1, 2 reading the fake width | refused by items 1, 2 alone | refused |
| `` manifest_rename/user_of_the_crate_without_the_glob `` | none | `` refused with cannot resolve a prelude import `` | refused |
| `` manifest_rename/user_of_the_crate_without_the_glob ``, fake width set to 64 | no item reading the fake width | refused with the same texts | refused |
| `` manifest_rename/user_of_the_crate_with_a_bare_prelude `` | none | `` refused with cannot resolve a prelude import `` | refused |
| `` manifest_rename/user_of_the_crate_with_a_bare_prelude ``, fake width set to 64 | no item reading the fake width | refused with the same texts | refused |
| `` manifest_rename/user_of_the_crate_with_an_empty_prelude_naming_assert `` | none | `` refused with cannot find macro `assert` in this scope `` | refused |
| `` manifest_rename/user_of_the_crate_with_an_empty_prelude_naming_assert ``, fake width set to 64 | no item reading the fake width | refused with the same texts | refused |
| `` manifest_rename/user_of_the_crate_with_a_bare_prelude_naming_assert `` | none | `` refused with cannot resolve a prelude import; cannot find macro `assert` in this scope `` | refused |
| `` manifest_rename/user_of_the_crate_with_a_bare_prelude_naming_assert ``, fake width set to 64 | no item reading the fake width | refused with the same texts | refused |
| `` manifest_rename/a_std_dependent_of_the_crate_without_the_glob `` | 2 const assert; reading the fake width: items 1, 2 | `` builds `` | builds |
| `` manifest_rename/a_std_dependent_of_the_crate_without_the_glob ``, item 1 flipped | const assert: `` const _: () = assert!(W != 8, "the manifest rename did not take over `::core`"); `` | refused by that item alone | refused |
| `` manifest_rename/a_std_dependent_of_the_crate_without_the_glob ``, item 2 flipped | const assert: `` W == usize::BITS, `` | refused by that item alone | refused |
| `` manifest_rename/a_std_dependent_of_the_crate_without_the_glob ``, fake width set to 64 | items 1, 2 reading the fake width | refused by items 1, 2 alone | refused |
| `` the_shipped_spelling_alone_is_silent.rs `` | none | `` builds, and checks no value `` | builds |
| `` the_bare_spelling_without_the_leading_colon_is_shadowed.rs `` | 1 const assert; reading the fake width: item 1 | `` refused with the bare spelling read the `mod core` shadow's 8 rather than the host's pointer width `` | refused |
| `` the_bare_spelling_without_the_leading_colon_is_shadowed.rs ``, fake width set to 64 | item 1 reading the fake width | builds | builds |
| harness self-check, `` selfcheck/holds_either_way_round.rs `` | `` W == 8 \|\| true `` | flagged by flip, premise | flagged by flip, premise |
| harness self-check, `` selfcheck/checks_nothing.rs `` | no item | flagged by structure | flagged by structure |
| harness self-check, `` selfcheck/no_message.rs `` | `` assert!(W == 8) ``, no message | flagged by structure | flagged by structure |
| harness self-check, `` selfcheck/an_escape_the_harness_cannot_match.rs `` | a message holding `\n` | flagged by structure | flagged by structure |
| harness self-check, `` selfcheck/an_item_declared_in_neither_list.rs `` | item 2 in neither list | flagged by structure | flagged by structure |
| harness self-check, `` selfcheck/an_item_declared_in_both_lists.rs `` | item 1 in both lists | flagged by structure | flagged by structure |
| harness self-check, `` selfcheck/two_items_sharing_a_message.rs `` | two items, one message | flagged by structure | flagged by structure |
| harness self-check, `` selfcheck/declared_to_check_nothing_and_checks.rs `` | declared `--checks-nothing`, holds an item | flagged by structure | flagged by structure |
| harness self-check, `` selfcheck/compares_the_width_with_itself.rs `` | `` W == W `` | flagged by premise | flagged by premise |
| harness self-check, `` selfcheck/a_true_comparison_before_the_width.rs `` | `` 1 == 1 && W == W `` | flagged by premise | flagged by premise |
| harness self-check, `` selfcheck/compares_the_width_with_another_constant.rs `` | `` W != 7 ``, the fake width 8 | flagged by premise | flagged by premise |
| harness self-check, `` selfcheck/a_resisted_read_compared_with_itself.rs `` | `` W == W ``, W read past the fake width | flagged by spelling | flagged by spelling |
| harness self-check, `` selfcheck/the_shipped_spelling_old_assertion.rs `` | two reads of the real width, no shadow | flagged by premise, spelling | flagged by premise, spelling |
| harness self-check, `` selfcheck/a_read_of_the_fake_width_declared_as_read_past_it.rs `` | `` W == 8 ``, W read through the shadow, declared `--resists` | flagged by premise, spelling | flagged by premise, spelling |
| harness self-check, `` selfcheck/a_vacuous_conjunct_beside_a_real_one.rs `` | `` W == 8 && W == W `` | flagged by none | flagged by none |
| harness self-check, `` selfcheck/an_escaped_quote_in_the_message.rs `` | a message holding `\"` | flagged by none | flagged by none |
| harness self-check, `` selfcheck/an_equals_sign_in_a_string_before_the_comparison.rs `` | a string holding `==` before the comparison | flagged by none | flagged by none |

unexpected outcomes: 0
