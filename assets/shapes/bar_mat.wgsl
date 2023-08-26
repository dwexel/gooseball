#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput

struct CustomMaterial {
	color: vec4<f32>,
	percentage: f32,
	corner_radius: f32
}

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

// use barycentric somehow?



@fragment
fn fragment(mesh: MeshVertexOutput) -> @location(0) vec4<f32> {
	let output_color: vec4<f32> = material.color;
	let clear_color = vec4<f32>(0.0);
	let corner_radius = material.corner_radius;

	// y-aspect 0.8
	// x-aspect 0.5

	// move the corner

	// offset is a portion of mesh size
	// (2 * offset) has to be less than (2 * the corner radius)
	let offset_y = 0.3;

	if mesh.uv.x > material.percentage {
		return clear_color;
	}
	else if mesh.uv.y < offset_y {
		return clear_color;
	}
	else if mesh.uv.y > 1.0 - offset_y {
		return clear_color;
	}
	else if 
		mesh.uv.x < corner_radius && 
		mesh.uv.y < corner_radius + offset_y && 
		// get length from corner to pixel
		length(mesh.uv - vec2<f32>(corner_radius, corner_radius + offset_y)) > corner_radius {
			return clear_color;
	}
	else if 
		mesh.uv.x < corner_radius && 
		mesh.uv.y > 1.0 - corner_radius - offset_y &&
		length(mesh.uv - vec2<f32>(corner_radius, 1.0 - corner_radius - offset_y)) > corner_radius {
			return clear_color;
	}
	else if 
		mesh.uv.x > 1.0 - corner_radius &&
		mesh.uv.y > 1.0 - corner_radius - offset_y &&
		length(mesh.uv - vec2<f32>(1.0 - corner_radius, 1.0 - corner_radius - offset_y)) > corner_radius {
			return clear_color;
	}
	else if 
		mesh.uv.x > 1.0 - corner_radius &&
		mesh.uv.y < corner_radius + offset_y &&
		length(mesh.uv - vec2<f32>(1.0 - corner_radius, corner_radius + offset_y)) > corner_radius {
			return clear_color;
	}
	else {
		return output_color;
	}
}