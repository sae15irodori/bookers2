class Book < ApplicationRecord
  belongs_to :user
  
  validates :title, presence: true#バリデーション
  validates :body, presence: true#バリデーション
end
