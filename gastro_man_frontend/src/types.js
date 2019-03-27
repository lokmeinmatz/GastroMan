export class User {
  constructor(user_name = '', first_name = '', last_name = '', permissions = []) {
    this.user_name = user_name
    this.first_name = first_name
    this.last_name = last_name
    this.permissions = permissions
    this.password = ''
  }
}