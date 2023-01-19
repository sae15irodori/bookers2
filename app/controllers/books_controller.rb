class BooksController < ApplicationController
  def index
    @book = Book.new#入力ﾌｫｰﾑ
  end

  def create
    @book = Book.new(book_params)
    if @book.save                 #ﾃﾞｰﾀ保存されたら
      redirect_to books_show_path#詳細ﾍﾟｰｼﾞへ飛ぶ
    else                        #ﾃﾞｰﾀ保存されなかったら
      render :index#indexﾍﾟｰｼﾞを再表示
    end
  end

  def show
  end

  private
    def book_params
      params.require(:book).permit(:title, :body)
    end

end
