class BooksController < ApplicationController
  def index
    @books = Book.all
    @book = Book.new#入力ﾌｫｰﾑ
  end

  def create
    @book = Book.new(book_params)
    @book.user_id = current_user.id#ﾌｫｰﾑに入力済ﾃﾞｰﾀのﾕｰｻﾞidにﾛｸﾞｲﾝ中のﾕｰｻﾞｰid代入
    if @book.save                 #ﾃﾞｰﾀ保存されたら
      flash[:notice] = "You have created book successfully."#ﾌﾗｯｼｭﾒｯｾｰｼﾞ
      redirect_to book_path(@book.id)#詳細ﾍﾟｰｼﾞへ飛ぶ
    else                        #ﾃﾞｰﾀ保存されなかったら
      @books = Book.all
      render :index#indexﾍﾟｰｼﾞを再表示
    end
  end

  def show
    @book= Book.find(params[:id])#特定の投稿ﾃﾞｰﾀのid取得
  end

  def destroy
    @book = Book.find(params[:id])
    @book.destroy
    redirect_to books_path
  end

  def edit
    @book = Book.find(params[:id])
  end

  private
    def book_params
      params.require(:book).permit(:title, :body)
    end

end
