class BooksController < ApplicationController
  def index
    @book = Book.new#入力ﾌｫｰﾑ
  end

  def create
    book = Book.new(book_params)
    book.save
    redirect_to books_show_path#詳細ﾍﾟｰｼﾞへ飛ぶ
  end

  def show
  end

  private
    def book_params
      params.require(:book).permit(:title, :body)
    end

end
