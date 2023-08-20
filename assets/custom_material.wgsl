#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput



struct CustomMaterial {
	color: vec4<f32>,
	radius: f32,
	percentage: f32
}

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

// angle = acos(adjacent / hypotenuse)
// is on the range [0, pi]

@fragment
fn fragment(mesh: MeshVertexOutput) -> @location(0) vec4<f32> {
   var output_color: vec4<f32> = material.color;
	var clear_color = vec4<f32>(0.0);

	var center = vec2<f32>(0.5, 0.5);
	var center_to_pix = mesh.uv - center;

	// length of the hypotenuse
	var hypotenuse = sqrt(pow(center_to_pix.x, 2.0) + pow(center_to_pix.y, 2.0));

	if hypotenuse <= material.radius 
	{
		// soft edge
		// output_color.a = smoothstep(1., 0.5, pow((hypotenuse / material.radius), 4.));

		// output_color.a = smoothstep(
		// 	output_color.a, 
		// 	0.0, 
		// 	pow(
		// 		((hypotenuse / material.radius) - 0.90) / 0.10, 
		// 		3.0
		// 	)
		// );

		output_color.a = smoothstep(
			output_color.a,
			0.0,
			pow(
				min(0.10, hypotenuse / material.radius - 0.90) / 0.10,
				3.0
			)
		);

		if material.percentage <= 0.5
		{
			if center_to_pix.y < 0.0
			{
				return clear_color;
			}
			if acos(center_to_pix.x / hypotenuse) < material.percentage * 2.0 * 3.1415
			{
				return output_color;
			}
		}
		else 
		{
			if center_to_pix.y > 0.0
			{
				return output_color;
			}
			if acos(center_to_pix.x / hypotenuse) > (1.0 - material.percentage) * 2.0 * 3.1415
			{
				return output_color;
			}
		}
	
	}

	return clear_color;
}