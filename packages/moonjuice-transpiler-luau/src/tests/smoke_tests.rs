#[cfg(test)]
mod tests {
  use crate::snapshot;
  use indoc::indoc;

  snapshot!(
    should_transpile_double_example,
    indoc! {"
      def double = fn(value) value * 2 end

      5 |> double |> print
    "}
  );

  snapshot!(should_transpile_issue_9, "(do 5 end) |> print");

  snapshot!(
    should_transpile_complex_unpack,
    indoc! {"
    export createWeapon = fn(
      {
        .visuals = {
          .worldModel,
          .worldModelOffset,
          .viewModel,
          .viewModelOffset
        },
        .weapon = {
          .isAutomatic,
          .shouldReloadAutomatically,
          .secondsPerRound,
          .roundsPerReload,
          .initialNumReloads,
          .reloadTimeSeconds
        },
        .callbacks = {
          .onShoot
        }
      },
      {
        .player,
        .parentEntity,
        .excludedColliders,
      }
    )
      print('hello world!')
    end
    "}
  );
}
