class AddVideoToBooks < ActiveRecord::Migration[6.1]
  def change
    add_column :books, :video, :string
  end
end
