# Access Layer TODOs with Security

## 1. User
- [x] Create user (`create_user`)
- [x] Get user by ID (`get_user_by_id`) – ensure only self/admin can access // i used session token for this, and we get the user only by using session token
- [-] Get user by email (`get_user_by_email`) – ensure only self/admin // i dont think its a gud idea at the end, maybe get users by username or by date ?
- [x] Update user (`update_user`) – self/admin only (so after all, i  think we can make a patch for user only, and we remove the un authorized fields)
- [x] Delete user (`delete_user`) – admin only // soft delet for sure
- [x] List users (`list_users`) – admin only

---

## 2. Workspace
- [x] Create workspace (`create_workspace`) – owner = current user
- [x] Get workspace by ID (`get_workspace_by_id`) – only owner can access
- [x] Update workspace (`update_workspace`) – only owner
- [ ] Delete workspace (`delete_workspace`) – only owner
- [ ] List workspaces by user (`list_workspaces_by_user`) – only self

---

## 3. Base
- [ ] Create base (`create_base`) – workspace owner only
- [ ] Get base by ID (`get_base_by_id`) – only workspace owner
- [ ] Update base (`update_base`) – only workspace owner
- [ ] Delete base (`delete_base`) – only workspace owner
- [ ] List bases by workspace (`list_bases_by_workspace`) – only workspace owner

---

## 4. Table
- [ ] Create table (`create_table`) – only workspace owner
- [ ] Get table by ID (`get_table_by_id`) – only workspace owner
- [ ] Update table (`update_table`) – only workspace owner
- [ ] Delete table (`delete_table`) – only workspace owner
- [ ] List tables by base (`list_tables_by_base`) – only workspace owner

---

## 5. Field
- [ ] Create field (`create_field`) – only workspace owner
- [ ] Get field by ID (`get_field_by_id`) – only workspace owner
- [ ] Update field (`update_field`) – only workspace owner
- [ ] Delete field (`delete_field`) – only workspace owner
- [ ] List fields by table (`list_fields_by_table`) – only workspace owner

---

## 6. Record
- [ ] Create record (`create_record`) – only workspace owner
- [ ] Get record by ID (`get_record_by_id`) – only workspace owner
- [ ] Update record (`update_record`) – only workspace owner
- [ ] Delete record (`delete_record`) – only workspace owner
- [ ] List records by table (`list_records_by_table`) – only workspace owner

---

## 7. Cell
- [ ] Create cell (`create_cell`) – only workspace owner
- [ ] Get cell by ID (`get_cell_by_id`) – only workspace owner
- [ ] Update cell (`update_cell`) – only workspace owner
- [ ] Delete cell (`delete_cell`) – only workspace owner
- [ ] List cells by record (`list_cells_by_record`) – only workspace owner
- [ ] List cells by field (`list_cells_by_field`) – only workspace owner

---

## 8. Identity
- [ ] Create identity (`create_identity`) – only self/admin
- [ ] Get identity by ID (`get_identity_by_id`) – only self/admin
- [ ] Get identity by external ID (`get_identity_by_external_id`) – only self/admin
- [ ] Delete identity (`delete_identity`) – only self/admin
- [ ] List identities by user (`list_identities_by_user`) – only self/admin

---

## 9. Relation
- [ ] Create relation (`create_relation`) – only workspace owner
- [ ] Get relation by ID (`get_relation_by_id`) – only workspace owner
- [ ] Delete relation (`delete_relation`) – only workspace owner
- [ ] List relations from record (`list_relations_from_record`) – only workspace owner
- [ ] List relations to record (`list_relations_to_record`) – only workspace owner

