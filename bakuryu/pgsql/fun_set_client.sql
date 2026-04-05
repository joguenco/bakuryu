CREATE OR REPLACE FUNCTION public.fun_set_client(p_name varchar, p_token varchar, p_path varchar)
	RETURNS int4
	LANGUAGE plpgsql
AS $function$
    DECLARE
		v_inserted_token	integer;
	BEGIN
        INSERT INTO access_tokens (token)
        VALUES(p_token)
        returning id into v_inserted_token;

    	INSERT INTO entities (name, folder_path, access_token_id)
    	VALUES(p_name, p_path, v_inserted_token);

        return v_inserted_token;
	END;
$function$
;
