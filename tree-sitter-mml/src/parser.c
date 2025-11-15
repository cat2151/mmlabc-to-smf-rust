#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 22
#define LARGE_STATE_COUNT 5
#define SYMBOL_COUNT 26
#define ALIAS_COUNT 0
#define TOKEN_COUNT 14
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 1

enum {
  anon_sym_SQUOTE = 1,
  sym_note = 2,
  anon_sym_PLUS = 3,
  anon_sym_DASH = 4,
  sym_rest = 5,
  sym_octave_up = 6,
  sym_octave_down = 7,
  anon_sym_o = 8,
  aux_sym_octave_set_token1 = 9,
  anon_sym_l = 10,
  anon_sym_AT = 11,
  anon_sym_t = 12,
  anon_sym_v = 13,
  sym_source_file = 14,
  sym__item = 15,
  sym_chord = 16,
  sym_note_with_modifier = 17,
  sym_modifier = 18,
  sym_octave_set = 19,
  sym_length_set = 20,
  sym_program_change = 21,
  sym_tempo_set = 22,
  sym_velocity_set = 23,
  aux_sym_source_file_repeat1 = 24,
  aux_sym_chord_repeat1 = 25,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_SQUOTE] = "'",
  [sym_note] = "note",
  [anon_sym_PLUS] = "+",
  [anon_sym_DASH] = "-",
  [sym_rest] = "rest",
  [sym_octave_up] = "octave_up",
  [sym_octave_down] = "octave_down",
  [anon_sym_o] = "o",
  [aux_sym_octave_set_token1] = "octave_set_token1",
  [anon_sym_l] = "l",
  [anon_sym_AT] = "@",
  [anon_sym_t] = "t",
  [anon_sym_v] = "v",
  [sym_source_file] = "source_file",
  [sym__item] = "_item",
  [sym_chord] = "chord",
  [sym_note_with_modifier] = "note_with_modifier",
  [sym_modifier] = "modifier",
  [sym_octave_set] = "octave_set",
  [sym_length_set] = "length_set",
  [sym_program_change] = "program_change",
  [sym_tempo_set] = "tempo_set",
  [sym_velocity_set] = "velocity_set",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_chord_repeat1] = "chord_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_SQUOTE] = anon_sym_SQUOTE,
  [sym_note] = sym_note,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_DASH] = anon_sym_DASH,
  [sym_rest] = sym_rest,
  [sym_octave_up] = sym_octave_up,
  [sym_octave_down] = sym_octave_down,
  [anon_sym_o] = anon_sym_o,
  [aux_sym_octave_set_token1] = aux_sym_octave_set_token1,
  [anon_sym_l] = anon_sym_l,
  [anon_sym_AT] = anon_sym_AT,
  [anon_sym_t] = anon_sym_t,
  [anon_sym_v] = anon_sym_v,
  [sym_source_file] = sym_source_file,
  [sym__item] = sym__item,
  [sym_chord] = sym_chord,
  [sym_note_with_modifier] = sym_note_with_modifier,
  [sym_modifier] = sym_modifier,
  [sym_octave_set] = sym_octave_set,
  [sym_length_set] = sym_length_set,
  [sym_program_change] = sym_program_change,
  [sym_tempo_set] = sym_tempo_set,
  [sym_velocity_set] = sym_velocity_set,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_chord_repeat1] = aux_sym_chord_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_SQUOTE] = {
    .visible = true,
    .named = false,
  },
  [sym_note] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [sym_rest] = {
    .visible = true,
    .named = true,
  },
  [sym_octave_up] = {
    .visible = true,
    .named = true,
  },
  [sym_octave_down] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_o] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_octave_set_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_l] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_t] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_v] = {
    .visible = true,
    .named = false,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym__item] = {
    .visible = false,
    .named = true,
  },
  [sym_chord] = {
    .visible = true,
    .named = true,
  },
  [sym_note_with_modifier] = {
    .visible = true,
    .named = true,
  },
  [sym_modifier] = {
    .visible = true,
    .named = true,
  },
  [sym_octave_set] = {
    .visible = true,
    .named = true,
  },
  [sym_length_set] = {
    .visible = true,
    .named = true,
  },
  [sym_program_change] = {
    .visible = true,
    .named = true,
  },
  [sym_tempo_set] = {
    .visible = true,
    .named = true,
  },
  [sym_velocity_set] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_chord_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(1);
      if (lookahead == '\'') ADVANCE(2);
      if (lookahead == '+') ADVANCE(4);
      if (lookahead == '-') ADVANCE(5);
      if (lookahead == '<') ADVANCE(7);
      if (lookahead == '>') ADVANCE(8);
      if (lookahead == '@') ADVANCE(12);
      if (lookahead == 'l') ADVANCE(11);
      if (lookahead == 'o') ADVANCE(9);
      if (lookahead == 't') ADVANCE(13);
      if (lookahead == 'v') ADVANCE(14);
      if (lookahead == 'R' ||
          lookahead == 'r') ADVANCE(6);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(10);
      if (('A' <= lookahead && lookahead <= 'G') ||
          ('a' <= lookahead && lookahead <= 'g')) ADVANCE(3);
      END_STATE();
    case 1:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 2:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(sym_note);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(sym_rest);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(sym_octave_up);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(sym_octave_down);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_o);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(aux_sym_octave_set_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(10);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_l);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_t);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_v);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_SQUOTE] = ACTIONS(1),
    [sym_note] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [sym_rest] = ACTIONS(1),
    [sym_octave_up] = ACTIONS(1),
    [sym_octave_down] = ACTIONS(1),
    [anon_sym_o] = ACTIONS(1),
    [aux_sym_octave_set_token1] = ACTIONS(1),
    [anon_sym_l] = ACTIONS(1),
    [anon_sym_AT] = ACTIONS(1),
    [anon_sym_t] = ACTIONS(1),
    [anon_sym_v] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(20),
    [sym__item] = STATE(2),
    [sym_chord] = STATE(2),
    [sym_note_with_modifier] = STATE(2),
    [sym_octave_set] = STATE(2),
    [sym_length_set] = STATE(2),
    [sym_program_change] = STATE(2),
    [sym_tempo_set] = STATE(2),
    [sym_velocity_set] = STATE(2),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_SQUOTE] = ACTIONS(5),
    [sym_note] = ACTIONS(7),
    [sym_rest] = ACTIONS(9),
    [sym_octave_up] = ACTIONS(9),
    [sym_octave_down] = ACTIONS(9),
    [anon_sym_o] = ACTIONS(11),
    [anon_sym_l] = ACTIONS(13),
    [anon_sym_AT] = ACTIONS(15),
    [anon_sym_t] = ACTIONS(17),
    [anon_sym_v] = ACTIONS(19),
  },
  [2] = {
    [sym__item] = STATE(3),
    [sym_chord] = STATE(3),
    [sym_note_with_modifier] = STATE(3),
    [sym_octave_set] = STATE(3),
    [sym_length_set] = STATE(3),
    [sym_program_change] = STATE(3),
    [sym_tempo_set] = STATE(3),
    [sym_velocity_set] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(21),
    [anon_sym_SQUOTE] = ACTIONS(5),
    [sym_note] = ACTIONS(7),
    [sym_rest] = ACTIONS(23),
    [sym_octave_up] = ACTIONS(23),
    [sym_octave_down] = ACTIONS(23),
    [anon_sym_o] = ACTIONS(11),
    [anon_sym_l] = ACTIONS(13),
    [anon_sym_AT] = ACTIONS(15),
    [anon_sym_t] = ACTIONS(17),
    [anon_sym_v] = ACTIONS(19),
  },
  [3] = {
    [sym__item] = STATE(3),
    [sym_chord] = STATE(3),
    [sym_note_with_modifier] = STATE(3),
    [sym_octave_set] = STATE(3),
    [sym_length_set] = STATE(3),
    [sym_program_change] = STATE(3),
    [sym_tempo_set] = STATE(3),
    [sym_velocity_set] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(25),
    [anon_sym_SQUOTE] = ACTIONS(27),
    [sym_note] = ACTIONS(30),
    [sym_rest] = ACTIONS(33),
    [sym_octave_up] = ACTIONS(33),
    [sym_octave_down] = ACTIONS(33),
    [anon_sym_o] = ACTIONS(36),
    [anon_sym_l] = ACTIONS(39),
    [anon_sym_AT] = ACTIONS(42),
    [anon_sym_t] = ACTIONS(45),
    [anon_sym_v] = ACTIONS(48),
  },
  [4] = {
    [sym_modifier] = STATE(7),
    [ts_builtin_sym_end] = ACTIONS(51),
    [anon_sym_SQUOTE] = ACTIONS(51),
    [sym_note] = ACTIONS(51),
    [anon_sym_PLUS] = ACTIONS(53),
    [anon_sym_DASH] = ACTIONS(53),
    [sym_rest] = ACTIONS(51),
    [sym_octave_up] = ACTIONS(51),
    [sym_octave_down] = ACTIONS(51),
    [anon_sym_o] = ACTIONS(51),
    [anon_sym_l] = ACTIONS(51),
    [anon_sym_AT] = ACTIONS(51),
    [anon_sym_t] = ACTIONS(51),
    [anon_sym_v] = ACTIONS(51),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 1,
    ACTIONS(55), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      sym_rest,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [14] = 1,
    ACTIONS(57), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      sym_rest,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [28] = 1,
    ACTIONS(59), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      sym_rest,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [42] = 1,
    ACTIONS(61), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      sym_rest,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [56] = 1,
    ACTIONS(63), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      sym_rest,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [70] = 1,
    ACTIONS(65), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      sym_rest,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [84] = 1,
    ACTIONS(67), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      sym_rest,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [98] = 1,
    ACTIONS(69), 11,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      sym_rest,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
      anon_sym_l,
      anon_sym_AT,
      anon_sym_t,
      anon_sym_v,
  [112] = 3,
    ACTIONS(7), 1,
      sym_note,
    ACTIONS(71), 1,
      anon_sym_SQUOTE,
    STATE(14), 2,
      sym_note_with_modifier,
      aux_sym_chord_repeat1,
  [123] = 3,
    ACTIONS(73), 1,
      anon_sym_SQUOTE,
    ACTIONS(75), 1,
      sym_note,
    STATE(14), 2,
      sym_note_with_modifier,
      aux_sym_chord_repeat1,
  [134] = 2,
    ACTIONS(7), 1,
      sym_note,
    STATE(13), 2,
      sym_note_with_modifier,
      aux_sym_chord_repeat1,
  [142] = 1,
    ACTIONS(78), 1,
      aux_sym_octave_set_token1,
  [146] = 1,
    ACTIONS(80), 1,
      aux_sym_octave_set_token1,
  [150] = 1,
    ACTIONS(82), 1,
      aux_sym_octave_set_token1,
  [154] = 1,
    ACTIONS(84), 1,
      aux_sym_octave_set_token1,
  [158] = 1,
    ACTIONS(86), 1,
      ts_builtin_sym_end,
  [162] = 1,
    ACTIONS(88), 1,
      aux_sym_octave_set_token1,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(5)] = 0,
  [SMALL_STATE(6)] = 14,
  [SMALL_STATE(7)] = 28,
  [SMALL_STATE(8)] = 42,
  [SMALL_STATE(9)] = 56,
  [SMALL_STATE(10)] = 70,
  [SMALL_STATE(11)] = 84,
  [SMALL_STATE(12)] = 98,
  [SMALL_STATE(13)] = 112,
  [SMALL_STATE(14)] = 123,
  [SMALL_STATE(15)] = 134,
  [SMALL_STATE(16)] = 142,
  [SMALL_STATE(17)] = 146,
  [SMALL_STATE(18)] = 150,
  [SMALL_STATE(19)] = 154,
  [SMALL_STATE(20)] = 158,
  [SMALL_STATE(21)] = 162,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [27] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(15),
  [30] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(4),
  [33] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(3),
  [36] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(21),
  [39] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(16),
  [42] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(17),
  [45] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(18),
  [48] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(19),
  [51] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_note_with_modifier, 1),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [55] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_length_set, 2),
  [57] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_modifier, 1),
  [59] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_note_with_modifier, 2),
  [61] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_octave_set, 2),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_program_change, 2),
  [65] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tempo_set, 2),
  [67] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_velocity_set, 2),
  [69] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chord, 3),
  [71] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [73] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_chord_repeat1, 2),
  [75] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_chord_repeat1, 2), SHIFT_REPEAT(4),
  [78] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [80] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [82] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [84] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [86] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [88] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_mml(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
