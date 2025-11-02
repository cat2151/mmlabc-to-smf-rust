module.exports = grammar({
  name: 'mml',

  rules: {
    source_file: $ => repeat($._item),

    _item: $ => choice(
      $.note,
    ),

    note: $ => /[cdefgabCDEFGAB]/,
  }
});
