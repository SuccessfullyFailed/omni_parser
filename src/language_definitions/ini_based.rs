use crate::implement_ini_lang;



implement_ini_lang!(
	Ini,
	&|value| value.to_owned(),
	&|value| value.to_owned()
);

implement_ini_lang!(
	Toml,
	&|value| {
		let no_quote_required:bool = value.parse::<bool>().is_ok() || value.parse::<u64>().is_ok() || value.parse::<f64>().is_ok() || (value.trim().starts_with('{') && value.trim().ends_with('}')) || (value.trim().starts_with('[') && value.trim().ends_with(']'));
		if no_quote_required {
			value.to_owned()
		} else {
			format!("\"{}\"", value)
		}
	},
	&|value| {
		if value.trim().starts_with('"') || value.trim().ends_with('"') {
			value.trim()[1..value.trim().len() - 1].trim().to_owned()
		} else {
			value.to_owned()
		}
	}
);