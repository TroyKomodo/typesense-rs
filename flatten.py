# This is used to fix the typesense OpenAPI Spec because they use object types in query parameters which is not supported by our generator.

import yaml
import argparse

def flatten_query_parameters(input_file, output_file):
    with open(input_file, 'r') as file:
        spec = yaml.safe_load(file)

    # Iterate over the paths
    for path, path_item in spec.get('paths', {}).items():
        # Iterate over the operations (GET, POST, etc.)
        for operation in path_item.values():
            parameters = operation.get('parameters', [])
            new_parameters = []
            
            for param in parameters:
                # Check if the parameter is in query and is an object
                if param['in'] == 'query':
                    schema = param.get('schema', {})
                    
                    # If it's a $ref, resolve the reference and flatten it
                    if '$ref' in schema:
                        ref_path = schema['$ref'].replace('#/components/schemas/', '')
                        if ref_path in spec['components']['schemas']:
                            schema = spec['components']['schemas'][ref_path]

                    # If the schema is of type object, flatten its properties
                    if schema.get('type') == 'object' and 'properties' in schema:
                        for prop_name, prop_schema in schema['properties'].items():
                            new_param = {
                                'name': prop_name,
                                'in': 'query',
                                'required': prop_name in schema.get('required', []),
                                'schema': prop_schema
                            }
                            new_parameters.append(new_param)
                    else:
                        # Keep the parameter as is if it's not an object
                        new_parameters.append(param)
                else:
                    # Keep non-query parameters as is
                    new_parameters.append(param)

            # Replace parameters with the flattened ones
            operation['parameters'] = new_parameters

    # Save the modified spec
    with open(output_file, 'w') as output_file:
        yaml.dump(spec, output_file)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Flatten object-type query parameters in OpenAPI spec.")
    parser.add_argument("input_file", help="Path to the input OpenAPI spec file (YAML format).")
    parser.add_argument("output_file", help="Path to save the flattened OpenAPI spec file.")
    
    args = parser.parse_args()
    
    flatten_query_parameters(args.input_file, args.output_file)
