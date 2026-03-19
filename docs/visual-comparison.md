# Visual Comparison: mermaid_to_svg vs mermaid-cli

Side-by-side comparison of our pure Rust rendering (left) against the canonical mermaid-cli output (right).

## Table of Contents

- [block](#block)
- [c4](#c4)
- [class](#class)
- [er](#er)
- [flowchart](#flowchart)
- [gantt](#gantt)
- [gitgraph](#gitgraph)
- [info](#info)
- [journey](#journey)
- [kanban](#kanban)
- [mindmap](#mindmap)
- [packet](#packet)
- [pie](#pie)
- [quadrant](#quadrant)
- [radar](#radar)
- [requirement](#requirement)
- [sankey](#sankey)
- [sequence](#sequence)
- [state](#state)
- [timeline](#timeline)
- [xychart](#xychart)

## block

### Block 01 Basic Block

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_block_01_basic_block.svg" width="400"> | <img src="comparisons/ref_block_01_basic_block.svg" width="400"> |

### Block 02 Three Node Chain

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_block_02_three_node_chain.svg" width="400"> | <img src="comparisons/ref_block_02_three_node_chain.svg" width="400"> |

### Block 03 Columns Layout

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_block_03_columns_layout.svg" width="400"> | <img src="comparisons/ref_block_03_columns_layout.svg" width="400"> |

### Block 04 Standalone Nodes

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_block_04_standalone_nodes.svg" width="400"> | <img src="comparisons/ref_block_04_standalone_nodes.svg" width="400"> |

## c4

### C4 01 Basic C4 Context

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_c4_01_basic_c4_context.svg" width="400"> | <img src="comparisons/ref_c4_01_basic_c4_context.svg" width="400"> |

## class

### Class 01 Basic Classes

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_class_01_basic_classes.svg" width="400"> | <img src="comparisons/ref_class_01_basic_classes.svg" width="400"> |

### Class 02 Relationships

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_class_02_relationships.svg" width="400"> | <img src="comparisons/ref_class_02_relationships.svg" width="400"> |

## er

### Er 01 Basic Er

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_er_01_basic_er.svg" width="400"> | <img src="comparisons/ref_er_01_basic_er.svg" width="400"> |

## flowchart

### Flowchart 01 Basic Flowchart

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_01_basic_flowchart.svg" width="400"> | <img src="comparisons/ref_flowchart_01_basic_flowchart.svg" width="400"> |

### Flowchart 02 Node Shapes

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_02_node_shapes.svg" width="400"> | <img src="comparisons/ref_flowchart_02_node_shapes.svg" width="400"> |

### Flowchart 03 Edge Types

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_03_edge_types.svg" width="400"> | <img src="comparisons/ref_flowchart_03_edge_types.svg" width="400"> |

### Flowchart 04 Edge Labels

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_04_edge_labels.svg" width="400"> | <img src="comparisons/ref_flowchart_04_edge_labels.svg" width="400"> |

### Flowchart 05 Decision Flow

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_05_decision_flow.svg" width="400"> | <img src="comparisons/ref_flowchart_05_decision_flow.svg" width="400"> |

### Flowchart 06 Subgraphs

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_06_subgraphs.svg" width="400"> | <img src="comparisons/ref_flowchart_06_subgraphs.svg" width="400"> |

### Flowchart 07 Horizontal

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_07_horizontal.svg" width="400"> | <img src="comparisons/ref_flowchart_07_horizontal.svg" width="400"> |

### Flowchart 08 Complex Flow

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_08_complex_flow.svg" width="400"> | <img src="comparisons/ref_flowchart_08_complex_flow.svg" width="400"> |

### Flowchart 09 Pipeline

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_09_pipeline.svg" width="400"> | <img src="comparisons/ref_flowchart_09_pipeline.svg" width="400"> |

### Flowchart 10 State Machine

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_10_state_machine.svg" width="400"> | <img src="comparisons/ref_flowchart_10_state_machine.svg" width="400"> |

### Flowchart 11 All Shapes

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_11_all_shapes.svg" width="400"> | <img src="comparisons/ref_flowchart_11_all_shapes.svg" width="400"> |

### Flowchart 12 Long Labels

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_12_long_labels.svg" width="400"> | <img src="comparisons/ref_flowchart_12_long_labels.svg" width="400"> |

### Flowchart 13 Special Chars

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_13_special_chars.svg" width="400"> | <img src="comparisons/ref_flowchart_13_special_chars.svg" width="400"> |

### Flowchart 14 Mixed Shapes Flow

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_14_mixed_shapes_flow.svg" width="400"> | <img src="comparisons/ref_flowchart_14_mixed_shapes_flow.svg" width="400"> |

### Flowchart 15 Subroutine Focus

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_15_subroutine_focus.svg" width="400"> | <img src="comparisons/ref_flowchart_15_subroutine_focus.svg" width="400"> |

### Flowchart 16 All Edge Styles

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_16_all_edge_styles.svg" width="400"> | <img src="comparisons/ref_flowchart_16_all_edge_styles.svg" width="400"> |

### Flowchart 17 Mixed Edge Styles

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_17_mixed_edge_styles.svg" width="400"> | <img src="comparisons/ref_flowchart_17_mixed_edge_styles.svg" width="400"> |

### Flowchart 18 Edge Direction Combo

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_18_edge_direction_combo.svg" width="400"> | <img src="comparisons/ref_flowchart_18_edge_direction_combo.svg" width="400"> |

### Flowchart 19 Thick Emphasis

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_19_thick_emphasis.svg" width="400"> | <img src="comparisons/ref_flowchart_19_thick_emphasis.svg" width="400"> |

### Flowchart 20 Dotted Async

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_20_dotted_async.svg" width="400"> | <img src="comparisons/ref_flowchart_20_dotted_async.svg" width="400"> |

### Flowchart 21 Long Edge Labels

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_21_long_edge_labels.svg" width="400"> | <img src="comparisons/ref_flowchart_21_long_edge_labels.svg" width="400"> |

### Flowchart 22 Labels All Types

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_22_labels_all_types.svg" width="400"> | <img src="comparisons/ref_flowchart_22_labels_all_types.svg" width="400"> |

### Flowchart 23 Multi Branch Labels

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_23_multi_branch_labels.svg" width="400"> | <img src="comparisons/ref_flowchart_23_multi_branch_labels.svg" width="400"> |

### Flowchart 24 Conditional Labels

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_24_conditional_labels.svg" width="400"> | <img src="comparisons/ref_flowchart_24_conditional_labels.svg" width="400"> |

### Flowchart 25 Label Special Chars

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_25_label_special_chars.svg" width="400"> | <img src="comparisons/ref_flowchart_25_label_special_chars.svg" width="400"> |

### Flowchart 26 Nested Subgraphs

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_26_nested_subgraphs.svg" width="400"> | <img src="comparisons/ref_flowchart_26_nested_subgraphs.svg" width="400"> |

### Flowchart 27 Peer Subgraphs

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_27_peer_subgraphs.svg" width="400"> | <img src="comparisons/ref_flowchart_27_peer_subgraphs.svg" width="400"> |

### Flowchart 28 Subgraph Titles

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_28_subgraph_titles.svg" width="400"> | <img src="comparisons/ref_flowchart_28_subgraph_titles.svg" width="400"> |

### Flowchart 29 Cross Subgraph

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_29_cross_subgraph.svg" width="400"> | <img src="comparisons/ref_flowchart_29_cross_subgraph.svg" width="400"> |

### Flowchart 30 Large Subgraph

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_30_large_subgraph.svg" width="400"> | <img src="comparisons/ref_flowchart_30_large_subgraph.svg" width="400"> |

### Flowchart 31 Subgraph Directions

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_31_subgraph_directions.svg" width="400"> | <img src="comparisons/ref_flowchart_31_subgraph_directions.svg" width="400"> |

### Flowchart 32 Architecture

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_32_architecture.svg" width="400"> | <img src="comparisons/ref_flowchart_32_architecture.svg" width="400"> |

### Flowchart 33 Bottom To Top

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_33_bottom_to_top.svg" width="400"> | <img src="comparisons/ref_flowchart_33_bottom_to_top.svg" width="400"> |

### Flowchart 34 Right To Left

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_34_right_to_left.svg" width="400"> | <img src="comparisons/ref_flowchart_34_right_to_left.svg" width="400"> |

### Flowchart 35 Mixed Directions

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_35_mixed_directions.svg" width="400"> | <img src="comparisons/ref_flowchart_35_mixed_directions.svg" width="400"> |

### Flowchart 36 Complex Branching

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_36_complex_branching.svg" width="400"> | <img src="comparisons/ref_flowchart_36_complex_branching.svg" width="400"> |

### Flowchart 37 Git Workflow

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_37_git_workflow.svg" width="400"> | <img src="comparisons/ref_flowchart_37_git_workflow.svg" width="400"> |

### Flowchart 38 Microservices

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_38_microservices.svg" width="400"> | <img src="comparisons/ref_flowchart_38_microservices.svg" width="400"> |

### Flowchart 39 Event Driven

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_39_event_driven.svg" width="400"> | <img src="comparisons/ref_flowchart_39_event_driven.svg" width="400"> |

### Flowchart 40 Auth Flow

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_40_auth_flow.svg" width="400"> | <img src="comparisons/ref_flowchart_40_auth_flow.svg" width="400"> |

### Flowchart 41 Ecommerce Checkout

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_41_ecommerce_checkout.svg" width="400"> | <img src="comparisons/ref_flowchart_41_ecommerce_checkout.svg" width="400"> |

### Flowchart 42 Db Schema

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_42_db_schema.svg" width="400"> | <img src="comparisons/ref_flowchart_42_db_schema.svg" width="400"> |

### Flowchart 43 Api Lifecycle

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_43_api_lifecycle.svg" width="400"> | <img src="comparisons/ref_flowchart_43_api_lifecycle.svg" width="400"> |

### Flowchart 44 Error Handling

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_44_error_handling.svg" width="400"> | <img src="comparisons/ref_flowchart_44_error_handling.svg" width="400"> |

### Flowchart 45 Feature Flags

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_45_feature_flags.svg" width="400"> | <img src="comparisons/ref_flowchart_45_feature_flags.svg" width="400"> |

### Flowchart 46 Many Nodes

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_46_many_nodes.svg" width="400"> | <img src="comparisons/ref_flowchart_46_many_nodes.svg" width="400"> |

### Flowchart 47 Deep Branching

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_47_deep_branching.svg" width="400"> | <img src="comparisons/ref_flowchart_47_deep_branching.svg" width="400"> |

### Flowchart 48 Long Chain

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_48_long_chain.svg" width="400"> | <img src="comparisons/ref_flowchart_48_long_chain.svg" width="400"> |

### Flowchart 49 Dense Connections

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_49_dense_connections.svg" width="400"> | <img src="comparisons/ref_flowchart_49_dense_connections.svg" width="400"> |

### Flowchart 50 Style Statements

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_flowchart_50_style_statements.svg" width="400"> | <img src="comparisons/ref_flowchart_50_style_statements.svg" width="400"> |

## gantt

### Gantt 01 Basic Gantt

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_gantt_01_basic_gantt.svg" width="400"> | <img src="comparisons/ref_gantt_01_basic_gantt.svg" width="400"> |

## gitgraph

### Gitgraph 01 Basic Gitgraph

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_gitgraph_01_basic_gitgraph.svg" width="400"> | <img src="comparisons/ref_gitgraph_01_basic_gitgraph.svg" width="400"> |

## info

### Info 01 Basic Info

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_info_01_basic_info.svg" width="400"> | <img src="comparisons/ref_info_01_basic_info.svg" width="400"> |

## journey

### Journey 01 Basic Journey

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_journey_01_basic_journey.svg" width="400"> | <img src="comparisons/ref_journey_01_basic_journey.svg" width="400"> |

### Journey 02 Multi Actor Journey

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_journey_02_multi_actor_journey.svg" width="400"> | <img src="comparisons/ref_journey_02_multi_actor_journey.svg" width="400"> |

## kanban

### Kanban 01 Basic Kanban

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_kanban_01_basic_kanban.svg" width="400"> | <img src="comparisons/ref_kanban_01_basic_kanban.svg" width="400"> |

### Kanban 02 Assigned Kanban

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_kanban_02_assigned_kanban.svg" width="400"> | <img src="comparisons/ref_kanban_02_assigned_kanban.svg" width="400"> |

### Kanban 03 Priority Kanban

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_kanban_03_priority_kanban.svg" width="400"> | <img src="comparisons/ref_kanban_03_priority_kanban.svg" width="400"> |

### Kanban 04 Multi Column Kanban

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_kanban_04_multi_column_kanban.svg" width="400"> | <img src="comparisons/ref_kanban_04_multi_column_kanban.svg" width="400"> |

## mindmap

### Mindmap 01 Basic Mindmap

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_mindmap_01_basic_mindmap.svg" width="400"> | <img src="comparisons/ref_mindmap_01_basic_mindmap.svg" width="400"> |

## packet

### Packet 01 Basic Packet

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_packet_01_basic_packet.svg" width="400"> | <img src="comparisons/ref_packet_01_basic_packet.svg" width="400"> |

## pie

### Pie 01 Basic Pie

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_pie_01_basic_pie.svg" width="400"> | <img src="comparisons/ref_pie_01_basic_pie.svg" width="400"> |

## quadrant

### Quadrant 01 Basic Quadrant

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_quadrant_01_basic_quadrant.svg" width="400"> | <img src="comparisons/ref_quadrant_01_basic_quadrant.svg" width="400"> |

## radar

### Radar 01 Basic Radar

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_radar_01_basic_radar.svg" width="400"> | <img src="comparisons/ref_radar_01_basic_radar.svg" width="400"> |

## requirement

### Requirement 01 Basic Requirement

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_requirement_01_basic_requirement.svg" width="400"> | <img src="comparisons/ref_requirement_01_basic_requirement.svg" width="400"> |

## sankey

### Sankey 01 Basic Sankey

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_sankey_01_basic_sankey.svg" width="400"> | <img src="comparisons/ref_sankey_01_basic_sankey.svg" width="400"> |

## sequence

### Sequence 01 Basic Sequence

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_sequence_01_basic_sequence.svg" width="400"> | <img src="comparisons/ref_sequence_01_basic_sequence.svg" width="400"> |

### Sequence 02 Loops And Notes

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_sequence_02_loops_and_notes.svg" width="400"> | <img src="comparisons/ref_sequence_02_loops_and_notes.svg" width="400"> |

## state

### State 01 Basic State

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_state_01_basic_state.svg" width="400"> | <img src="comparisons/ref_state_01_basic_state.svg" width="400"> |

### State 02 Choice And Fork

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_state_02_choice_and_fork.svg" width="400"> | <img src="comparisons/ref_state_02_choice_and_fork.svg" width="400"> |

## timeline

### Timeline 01 Basic Timeline

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_timeline_01_basic_timeline.svg" width="400"> | <img src="comparisons/ref_timeline_01_basic_timeline.svg" width="400"> |

## xychart

### Xychart 01 Basic Xychart

| Ours (mermaid_to_svg) | Reference (mermaid-cli) |
|---|---|
| <img src="comparisons/our_xychart_01_basic_xychart.svg" width="400"> | <img src="comparisons/ref_xychart_01_basic_xychart.svg" width="400"> |
