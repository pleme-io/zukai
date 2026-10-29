def info_panel()
  cf_panel_info("Read this first.")
end

def every_panel()
  [
    cf_panel_info("x"),
    cf_panel_warning("x"),
    cf_panel_note("x"),
    cf_panel_tip("x"),
    cf_panel_success("x")
  ]
end

def rust_code()
  cf_code("rust", "fn x() {}")
end

def expand_with_tip()
  cf_expand("Show details", cf_panel_tip("use the API token"))
end

def green_status()
  cf_status("Green", "OK")
end

def toc_two_to_three()
  cf_toc(2, 3)
end

def two_tabs()
  cf_tabs([%w[Overview <p>over</p>], %w[Details <p>det</p>]])
end

def done_and_todo()
  cf_tasklist([[true, "done"], [false, "todo"]])
end

def two_columns()
  cf_two_column("<p>L</p>", "<p>R</p>")
end

def hello()
  cf_paragraph("hello")
end

def divider()
  cf_hr
end

def docs_link()
  cf_link("docs", "https://example.com")
end

def titled_page_link()
  cf_page_link("4112318475", "See demo")
end

def bare_page_link()
  cf_page_link("12345", nil)
end

def service_table()
  cf_table(%w[Service IP], [%w[vault 1.2.3.4], %w[auth 5.6.7.8]])
end

def ordered_body()
  cf_compose([cf_panel_info("a"), cf_hr, cf_panel_tip("b")])
end

def body_of(parts)
  cf_compose(parts)
end

def nested_layout()
  steps = cf_tasklist([[true, "render"], [false, "upload"]])
  cf_two_column(
    cf_expand("Show details", cf_panel_tip("use the API token")),
    cf_tabs(
      [["Code", cf_code("rust", "Png.render(&d, &theme)")], ["Steps", steps]]
    )
  )
end
