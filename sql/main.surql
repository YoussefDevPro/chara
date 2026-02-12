
-- USER 
DEFINE TABLE OVERWRITE user SCHEMAFULL;
DEFINE FIELD OVERWRITE created_at ON TABLE user TYPE datetime 
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON TABLE user TYPE datetime
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE is_deleted ON TABLE user TYPE bool 
DEFAULT false;
DEFINE FIELD OVERWRITE first_name ON TABLE user TYPE string;
DEFINE FIELD OVERWRITE last_name ON TABLE user TYPE string;
DEFINE FIELD OVERWRITE email ON TABLE user TYPE string
ASSERT string::is::email($value);
DEFINE FIELD OVERWRITE role ON TABLE user TYPE string 
DEFAULT 'user'
ASSERT $value INSIDE ['admin', 'user'];

-- SESSION 
DEFINE TABLE OVERWRITE session SCHEMAFULL;
DEFINE FIELD OVERWRITE token ON TABLE session TYPE string;
DEFINE FIELD OVERWRITE ip ON TABLE session TYPE string
ASSERT string::is::ip($value);
DEFINE FIELD OVERWRITE user_agent ON TABLE session TYPE string;
DEFINE FIELD OVERWRITE created_at ON TABLE session TYPE datetime
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE expires_at ON TABLE session TYPE datetime
VALUE time::now() + 5d READONLY;
DEFINE FIELD OVERWRITE user ON TABLE session TYPE record<user>;
DEFINE INDEX OVERWRITE user_unique ON TABLE session FIELDS user UNIQUE;

-- IDENTITY 
DEFINE TABLE OVERWRITE identity SCHEMAFULL;
DEFINE FIELD OVERWRITE created_at ON TABLE identity TYPE datetime 
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON TABLE identity TYPE datetime
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE is_deleted ON TABLE identity TYPE bool 
DEFAULT false;
DEFINE FIELD OVERWRITE user ON TABLE identity TYPE record<user>;
DEFINE FIELD OVERWRITE external_id ON TABLE identity TYPE string;
DEFINE INDEX OVERWRITE user_unique ON TABLE identity FIELDS user UNIQUE;
DEFINE INDEX OVERWRITE external_id_unique ON TABLE identity FIELDS user UNIQUE;

-- WORKSPACE 
DEFINE TABLE OVERWRITE workspace SCHEMAFULL;
DEFINE FIELD OVERWRITE created_at ON TABLE workspace TYPE datetime 
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON TABLE workspace TYPE datetime
VALUE time::now();
DEFINE FIELD OVERWRITE is_deleted ON TABLE workspace TYPE bool 
DEFAULT false;
DEFINE FIELD OVERWRITE owner ON TABLE workspace TYPE record<user>;
DEFINE FIELD OVERWRITE name ON TABLE workspace TYPE string;

-- WORKSPACE USER 
DEFINE TABLE OVERWRITE workspace_user SCHEMAFULL;
DEFINE FIELD OVERWRITE created_at ON TABLE workspace_user TYPE datetime 
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON TABLE workspace_user TYPE datetime
VALUE time::now();
DEFINE FIELD OVERWRITE is_deleted ON TABLE workspace_user TYPE bool 
DEFAULT false;
DEFINE FIELD OVERWRITE workspace_id ON TABLE workspace_user TYPE record<workspace>;
DEFINE FIELD OVERWRITE username ON TABLE workspace_user TYPE option<string> 
VALUE $value OR user.first_name;
DEFINE FIELD OVERWRITE user ON TABLE workspace_user TYPE record<user>;
DEFINE INDEX OVERWRITE unique_user_per_workspace 
    ON TABLE workspace_user 
    FIELDS user, workspace_id UNIQUE;

-- BASE 
DEFINE TABLE OVERWRITE base SCHEMAFULL;
DEFINE FIELD OVERWRITE created_at ON TABLE base TYPE datetime 
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON TABLE base TYPE datetime
VALUE time::now();
DEFINE FIELD OVERWRITE is_deleted ON TABLE base TYPE bool 
DEFAULT false;
DEFINE FIELD OVERWRITE name ON TABLE base TYPE string;

-- TABLE 
DEFINE TABLE OVERWRITE table SCHEMAFULL;
DEFINE FIELD OVERWRITE created_at ON TABLE table TYPE datetime 
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON TABLE table TYPE datetime
VALUE time::now();
DEFINE FIELD OVERWRITE is_deleted ON TABLE table TYPE bool 
DEFAULT false;
DEFINE FIELD OVERWRITE name ON TABLE base TYPE string;

-- WORKSPACE |-relation-> BASE 
DEFINE TABLE OVERWRITE have_base TYPE RELATION 
    IN workspace 
    OUT base 
    SCHEMAFULL;
DEFINE FIELD OVERWRITE created_at ON TABLE have_base TYPE datetime
    VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE is_public ON TABLE have_base TYPE bool;

-- BASE |-relation-> TABLE
DEFINE TABLE OVERWRITE have_table TYPE RELATION 
    IN base
    OUT table
    SCHEMAFULL;
DEFINE FIELD OVERWRITE created_at ON TABLE have_table TYPE datetime
    VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE is_public ON TABLE have_table TYPE bool;

-- FIELD 
DEFINE TABLE OVERWRITE field SCHEMAFULL;
DEFINE FIELD OVERWRITE created_at ON TABLE field TYPE datetime 
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON TABLE field TYPE datetime
VALUE time::now();
DEFINE FIELD OVERWRITE is_deleted ON TABLE field TYPE bool 
DEFAULT false;
DEFINE FIELD OVERWRITE is_primary ON TABLE field TYPE bool
DEFAULT true;
DEFINE FIELD OVERWRITE is_nullable ON TABLE field TYPE bool;
DEFINE FIELD OVERWRITE is_unique ON TABLE field TYPE bool;
DEFINE FIELD OVERWRITE name ON TABLE field TYPE string;
DEFINE FIELD OVERWRITE order ON TABLE field TYPE int;
DEFINE FIELD OVERWRITE description ON TABLE field TYPE option<string>;
DEFINE FIELD OVERWRITE config ON TABLE field TYPE object;

-- RECORD 
DEFINE TABLE OVERWRITE record SCHEMAFULL;
DEFINE FIELD OVERWRITE created_at ON TABLE record TYPE datetime 
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON TABLE record TYPE datetime
VALUE time::now();
DEFINE FIELD OVERWRITE is_deleted ON TABLE record TYPE bool 
DEFAULT false;

-- CELL 
DEFINE TABLE OVERWRITE cell SCHEMAFULL;
DEFINE FIELD OVERWRITE created_at ON TABLE cell TYPE datetime
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON TABLE cell TYPE datetime 
VALUE time::now();
DEFINE FIELD OVERWRITE is_deleted ON TABLE cell TYPE bool 
DEFAULT false;
DEFINE FIELD OVERWRITE value ON TABLE cell TYPE object;

-- RECORD & FIELD |-relation-> TABLE
DEFINE TABLE OVERWRITE have_cell TYPE RELATION 
    IN record | field
    OUT cell
    SCHEMAFULL;

-- TABLE |-relation-> FIELD
DEFINE TABLE OVERWRITE have_field TYPE RELATION 
    IN table
    OUT field
    SCHEMAFULL;
DEFINE FIELD OVERWRITE created_at ON TABLE have_field TYPE datetime
    VALUE time::now() READONLY;

-- TABLE |-relation-> RECORD 
DEFINE TABLE OVERWRITE have_record TYPE RELATION 
    IN table
    OUT record
    SCHEMAFULL;
DEFINE FIELD OVERWRITE created_at ON TABLE have_record TYPE datetime
    VALUE time::now() READONLY;


