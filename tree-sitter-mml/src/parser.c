#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 14
#define LARGE_STATE_COUNT 5
#define SYMBOL_COUNT 18
#define ALIAS_COUNT 0
#define TOKEN_COUNT 10
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
  sym_source_file = 10,
  sym__item = 11,
  sym_chord = 12,
  sym_note_with_modifier = 13,
  sym_modifier = 14,
  sym_octave_set = 15,
  aux_sym_source_file_repeat1 = 16,
  aux_sym_chord_repeat1 = 17,
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
  [sym_source_file] = "source_file",
  [sym__item] = "_item",
  [sym_chord] = "chord",
  [sym_note_with_modifier] = "note_with_modifier",
  [sym_modifier] = "modifier",
  [sym_octave_set] = "octave_set",
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
  [sym_source_file] = sym_source_file,
  [sym__item] = sym__item,
  [sym_chord] = sym_chord,
  [sym_note_with_modifier] = sym_note_with_modifier,
  [sym_modifier] = sym_modifier,
  [sym_octave_set] = sym_octave_set,
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
      if (lookahead == 'o') ADVANCE(9);
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
  },
  [1] = {
    [sym_source_file] = STATE(13),
    [sym__item] = STATE(2),
    [sym_chord] = STATE(2),
    [sym_note_with_modifier] = STATE(2),
    [sym_octave_set] = STATE(2),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_SQUOTE] = ACTIONS(5),
    [sym_note] = ACTIONS(7),
    [sym_rest] = ACTIONS(9),
    [sym_octave_up] = ACTIONS(9),
    [sym_octave_down] = ACTIONS(9),
    [anon_sym_o] = ACTIONS(11),
  },
  [2] = {
    [sym__item] = STATE(3),
    [sym_chord] = STATE(3),
    [sym_note_with_modifier] = STATE(3),
    [sym_octave_set] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(13),
    [anon_sym_SQUOTE] = ACTIONS(5),
    [sym_note] = ACTIONS(7),
    [sym_rest] = ACTIONS(15),
    [sym_octave_up] = ACTIONS(15),
    [sym_octave_down] = ACTIONS(15),
    [anon_sym_o] = ACTIONS(11),
  },
  [3] = {
    [sym__item] = STATE(3),
    [sym_chord] = STATE(3),
    [sym_note_with_modifier] = STATE(3),
    [sym_octave_set] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(17),
    [anon_sym_SQUOTE] = ACTIONS(19),
    [sym_note] = ACTIONS(22),
    [sym_rest] = ACTIONS(25),
    [sym_octave_up] = ACTIONS(25),
    [sym_octave_down] = ACTIONS(25),
    [anon_sym_o] = ACTIONS(28),
  },
  [4] = {
    [sym_modifier] = STATE(6),
    [ts_builtin_sym_end] = ACTIONS(31),
    [anon_sym_SQUOTE] = ACTIONS(31),
    [sym_note] = ACTIONS(31),
    [anon_sym_PLUS] = ACTIONS(33),
    [anon_sym_DASH] = ACTIONS(33),
    [sym_rest] = ACTIONS(31),
    [sym_octave_up] = ACTIONS(31),
    [sym_octave_down] = ACTIONS(31),
    [anon_sym_o] = ACTIONS(31),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 1,
    ACTIONS(35), 7,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      sym_rest,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
  [10] = 1,
    ACTIONS(37), 7,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      sym_rest,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
  [20] = 1,
    ACTIONS(39), 7,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      sym_rest,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
  [30] = 1,
    ACTIONS(41), 7,
      ts_builtin_sym_end,
      anon_sym_SQUOTE,
      sym_note,
      sym_rest,
      sym_octave_up,
      sym_octave_down,
      anon_sym_o,
  [40] = 3,
    ACTIONS(7), 1,
      sym_note,
    ACTIONS(43), 1,
      anon_sym_SQUOTE,
    STATE(10), 2,
      sym_note_with_modifier,
      aux_sym_chord_repeat1,
  [51] = 3,
    ACTIONS(45), 1,
      anon_sym_SQUOTE,
    ACTIONS(47), 1,
      sym_note,
    STATE(10), 2,
      sym_note_with_modifier,
      aux_sym_chord_repeat1,
  [62] = 2,
    ACTIONS(7), 1,
      sym_note,
    STATE(9), 2,
      sym_note_with_modifier,
      aux_sym_chord_repeat1,
  [70] = 1,
    ACTIONS(50), 1,
      aux_sym_octave_set_token1,
  [74] = 1,
    ACTIONS(52), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(5)] = 0,
  [SMALL_STATE(6)] = 10,
  [SMALL_STATE(7)] = 20,
  [SMALL_STATE(8)] = 30,
  [SMALL_STATE(9)] = 40,
  [SMALL_STATE(10)] = 51,
  [SMALL_STATE(11)] = 62,
  [SMALL_STATE(12)] = 70,
  [SMALL_STATE(13)] = 74,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [19] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(11),
  [22] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(4),
  [25] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(3),
  [28] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(12),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_note_with_modifier, 1),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_modifier, 1),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_note_with_modifier, 2),
  [39] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_octave_set, 2),
  [41] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chord, 3),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_chord_repeat1, 2),
  [47] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_chord_repeat1, 2), SHIFT_REPEAT(4),
  [50] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [52] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
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
