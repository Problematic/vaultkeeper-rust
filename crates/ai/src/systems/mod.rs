use legion::systems::schedule::Builder;

mod action_selection_system;

use action_selection_system::build_action_selection_system;

pub fn register_systems(sb: Builder) -> Builder {
  sb.add_system(build_action_selection_system())
}
