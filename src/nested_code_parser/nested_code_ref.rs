#[cfg(test)]
mod tests {
	use crate::{ NestedSegment, NestedSegmentIterator, NestedSegmentRef };


	
	#[test]
	fn test_nested_segment_creation() {
		let code_segment:NestedSegment = NestedSegment::new_code("test", "<test>", vec![], "</test>");
		let content_segment:NestedSegment = NestedSegment::new_contents("Hello");
		let whitespace_segment:NestedSegment = NestedSegment::new_contents("   ");

		assert!(matches!(code_segment, NestedSegment::Code(_, _)));
		assert!(matches!(content_segment, NestedSegment::Contents(_, _)));
		assert!(matches!(whitespace_segment, NestedSegment::WhiteSpace(_, _)));
	}

	#[test]
	fn test_nested_segment_sub_segments() {
		let inner:NestedSegment = NestedSegment::new_contents("inner");
		let outer:NestedSegment = NestedSegment::new_code("outer", "<outer>", vec![inner.clone()], "</outer>");

		if let NestedSegment::Code(_, code) = &outer {
			assert_eq!(code.sub_segments.len(), 1);
			assert_eq!(code.sub_segments[0], inner);
		} else {
			panic!("Expected Code variant");
		}
	}

	#[test]
	fn test_nested_segment_flattening() {
		let inner:NestedSegment = NestedSegment::new_contents("inner");
		let outer:NestedSegment = NestedSegment::new_code("outer", "<outer>", vec![inner.clone()], "</outer>");
		let flat:Vec<(usize, NestedSegment)> = outer.to_flat();

		assert_eq!(flat.len(), 2);
		assert!(matches!(flat[0].1, NestedSegment::Code(_, _)));
		assert!(matches!(flat[1].1, NestedSegment::Contents(_, _)));
	}

	#[test]
	fn test_nested_segment_iterator() {
		let inner:NestedSegment = NestedSegment::new_contents("inner");
		let outer:NestedSegment = NestedSegment::new_code("outer", "<outer>", vec![inner.clone()], "</outer>");
		let mut iter:NestedSegmentIterator<'_> = outer.iter();

		assert_eq!(iter.next().unwrap().type_name(), "outer");
		assert!(iter.next().unwrap().is_contents());
		assert!(iter.next().is_none());
	}

	#[test]
	fn test_nested_segment_ref() {
		let inner:NestedSegment = NestedSegment::new_contents("inner");
		let outer:NestedSegment = NestedSegment::new_code("outer", "<outer>", vec![inner.clone()], "</outer>");
		let seg_ref:NestedSegmentRef<'_> = NestedSegmentRef::new(&outer, vec![]);
		
		assert!(seg_ref.get().is_some());
		assert_eq!(seg_ref.get().unwrap().type_name(), "outer");
	}

	#[test]
	fn test_nested_segment_ref_child() {
		let inner:NestedSegment = NestedSegment::new_contents("inner");
		let outer:NestedSegment = NestedSegment::new_code("outer", "<outer>", vec![inner.clone()], "</outer>");
		let seg_ref:NestedSegmentRef<'_> = NestedSegmentRef::new(&outer, vec![]);

		let child_ref:NestedSegmentRef<'_> = seg_ref.child().unwrap();
		assert!(child_ref.get().unwrap().is_contents());
	}
}